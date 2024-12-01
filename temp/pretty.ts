export function checkIfParentIsDottedExpression(path: AstPath): boolean {
  const node = path.getNode();
  const parentNode = path.getParentNode();

  let result = false;
  // We're making an assumption here that `callParent` is always synchronous.
  // We're doing it because FastPath does not expose other ways to find the
  // parent name.
  let parentNodeName;
  let grandParentNodeName;
  path.callParent((innerPath) => {
    parentNodeName = innerPath.getName();
  });
  path.callParent((innerPath) => {
    grandParentNodeName = innerPath.getName();
  }, 1);
  if (parentNodeName === "dottedExpr") {
    result = true;
  } else if (
    node["@class"] === apexTypes.VARIABLE_EXPRESSION &&
    parentNode["@class"] === apexTypes.ARRAY_EXPRESSION &&
    grandParentNodeName === "dottedExpr"
  ) {
    // a
    //   .b[0]  // <- Node b here
    //   .c()
    // For this situation we want to flag b as a nested dotted expression,
    // so that we can make it part of the grand parent's group, even though
    // technically it's the grandchild of the dotted expression.
    result = true;
  }
  return result;
}

function handleDottedExpression(path: AstPath, print: printFn): Doc {
  const node = path.getNode();
  const dottedExpressionParts: Doc[] = [];
  const dottedExpressionDoc: Doc = path.call(print, "dottedExpr", "value");

  if (dottedExpressionDoc) {
    dottedExpressionParts.push(dottedExpressionDoc);
    if (shouldDottedExpressionBreak(path)) {
      dottedExpressionParts.push(softline);
    }
    if (node.isSafeNav) {
      dottedExpressionParts.push("?");
    }
    dottedExpressionParts.push(".");
    return dottedExpressionParts;
  }
  return "";
}

function handleInputParameters(path: AstPath, print: printFn): Doc[] {
  // In most cases, the descendant nodes inside `inputParameters` will create
  // their own groups. However, in certain circumstances (i.e. with binaryish
  // behavior), they rely on groups created by their parents. That's why we
  // wrap each inputParameter in a group here. See #693 for an example case.
  return path.map(print, "inputParameters").map((paramDoc) => group(paramDoc));
}

function handleMethodCallExpression(path: AstPath, print: printFn): Doc {
  const node = path.getNode();
  const parentNode = path.getParentNode();
  const nodeName = path.getName();
  const { dottedExpr } = node;
  const isParentDottedExpression = checkIfParentIsDottedExpression(path);
  const isDottedExpressionSoqlExpression =
    dottedExpr &&
    dottedExpr.value &&
    (dottedExpr.value["@class"] === APEX_TYPES.SOQL_EXPRESSION ||
      (dottedExpr.value["@class"] === APEX_TYPES.ARRAY_EXPRESSION &&
        dottedExpr.value.expr &&
        dottedExpr.value.expr["@class"] === APEX_TYPES.SOQL_EXPRESSION));
  const isDottedExpressionThisVariableExpression =
    dottedExpr &&
    dottedExpr.value &&
    dottedExpr.value["@class"] === APEX_TYPES.THIS_VARIABLE_EXPRESSION;
  const isDottedExpressionSuperVariableExpression =
    dottedExpr &&
    dottedExpr.value &&
    dottedExpr.value["@class"] === APEX_TYPES.SUPER_VARIABLE_EXPRESSION;

  const dottedExpressionDoc = handleDottedExpression(path, print);
  const nameDocs: Doc[] = path.map(print, "names");
  const paramDocs: Doc[] = handleInputParameters(path, print);

  const resultParamDoc =
    paramDocs.length > 0
      ? [softline, join([",", line], paramDocs), dedent(softline)]
      : "";

  const methodCallChainDoc = join(".", nameDocs);

  // Handling the array expression index.
  // Technically, in this statement: a()[b],
  // the method call expression is a child of the array expression.
  // However, for certain situation we need to print the [] part as part of
  // the group from the method call expression. For example:
  // a
  //   .b
  //   .c()[
  //     d.callMethod()
  //   ]
  // If we print the [] as part of the array expression, like we usually do,
  // the result will be:
  // a
  //   .b
  //   .c()[
  //   d.callMethod()
  // ]
  // Hence why we are deferring the printing of the [] part from handleArrayExpression
  // to here.
  let arrayIndexDoc: Doc = "";
  if (
    parentNode["@class"] === APEX_TYPES.ARRAY_EXPRESSION &&
    nodeName === "expr"
  ) {
    path.callParent((innerPath: AstPath) => {
      const withGroup = isParentDottedExpression || !!dottedExpressionDoc;

      arrayIndexDoc = handleArrayExpressionIndex(innerPath, print, withGroup);
    });
  }
  let resultDoc;
  const noGroup =
    // If this is a nested dotted expression, we do not want to group it,
    // since we want it to be part of the method call chain group, e.g:
    // a
    //   .b()  // <- this node here
    //   .c()  // <- this node here
    //   .d()
    isParentDottedExpression ||
    // If dotted expression is SOQL and this in inside a binaryish expression,
    // we shouldn't group it, otherwise there will be extraneous indentations,
    // for example:
    // Boolean a =
    //   [
    //     SELECT Id FROM Contact
    //   ].size() > 0
    (isDottedExpressionSoqlExpression && isBinaryish(parentNode)) ||
    // If dotted expression is a `super` or `this` variable expression, we
    // know that this is only one level deep and there's no need to group, e.g:
    // `this.simpleMethod();` or `super.simpleMethod();`
    isDottedExpressionThisVariableExpression ||
    isDottedExpressionSuperVariableExpression;
  if (noGroup) {
    resultDoc = [
      dottedExpressionDoc,
      methodCallChainDoc,
      "(",
      group(indent(resultParamDoc)),
      ")",
      arrayIndexDoc,
    ];
  } else {
    // This means it is the highest level method call expression,
    // and we do need to group and indent the expressions in it, e.g:
    // a
    //   .b()
    //   .c()
    //   .d()  // <- this node here
    resultDoc = group(
      indent([
        dottedExpressionDoc,
        // If there is no dottedExpr, we should group the method call chain
        // to have this effect:
        // a.callMethod(  // <- 2 names (a and callMethod)
        //   'a',
        //   'b'
        // )
        // Otherwise we don't want to group them, so that they're part of the
        // parent group. It will format this code:
        // a.b().c().callMethod('a', 'b') // <- 4 names (a, b, c, callMethod)
        // into this:
        // a.b()
        //   .c()
        //   .callMethod('a', 'b')
        dottedExpressionDoc ? methodCallChainDoc : group(methodCallChainDoc),
        "(",
        dottedExpressionDoc
          ? group(indent(resultParamDoc))
          : group(resultParamDoc),
        ")",
        arrayIndexDoc,
      ]),
    );
  }
  return resultDoc;
}

// a.b().c();
{
  "apex.jorje.semantic.compiler.parser.ParserOutput": {
    "internalErrors": [],
    "parseErrors": [],
    "unit": {
      "@class": "apex.jorje.data.ast.CompilationUnit$ClassDeclUnit",
      "body": {
        "@class": "apex.jorje.data.ast.ClassDecl",
        "loc": {
          "@class": "apex.jorje.data.IndexLocation",
          "startIndex": 0,
          "endIndex": 38,
          "line": 1,
          "column": 1
        },
        "modifiers": [],
        "name": {
          "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
          "loc": {
            "@class": "apex.jorje.data.IndexLocation",
            "startIndex": 6,
            "endIndex": 16,
            "line": 1,
            "column": 7
          },
          "value": "HelloWorld"
        },
        "typeArguments": {},
        "members": [
          {
            "@class": "apex.jorje.data.ast.BlockMember$StmntBlockMember",
            "stmnt": {
              "@class": "apex.jorje.data.ast.Stmnt$BlockStmnt",
              "loc": {
                "@class": "apex.jorje.data.IndexLocation",
                "startIndex": 19,
                "endIndex": 36,
                "line": 2,
                "column": 1
              },
              "stmnts": [
                {
                  "@class": "apex.jorje.data.ast.Stmnt$ExpressionStmnt",
                  "loc": {
                    "@class": "apex.jorje.data.IndexLocation",
                    "startIndex": 30,
                    "endIndex": 34,
                    "line": 3,
                    "column": 10
                  },
                  "expr": {
                    "@class": "apex.jorje.data.ast.Expr$MethodCallExpr",
                    "dottedExpr": {
                      "value": {
                        "@class": "apex.jorje.data.ast.Expr$MethodCallExpr",
                        "dottedExpr": {},
                        "isSafeNav": false,
                        "names": [
                          {
                            "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                            "loc": {
                              "@class": "apex.jorje.data.IndexLocation",
                              "startIndex": 22,
                              "endIndex": 23,
                              "line": 3,
                              "column": 2
                            },
                            "value": "a"
                          },
                          {
                            "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                            "loc": {
                              "@class": "apex.jorje.data.IndexLocation",
                              "startIndex": 24,
                              "endIndex": 25,
                              "line": 3,
                              "column": 4
                            },
                            "value": "k"
                          },
                          {
                            "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                            "loc": {
                              "@class": "apex.jorje.data.IndexLocation",
                              "startIndex": 26,
                              "endIndex": 27,
                              "line": 3,
                              "column": 6
                            },
                            "value": "b"
                          }
                        ],
                        "inputParameters": []
                      }
                    },
                    "isSafeNav": false,
                    "names": [
                      {
                        "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                        "loc": {
                          "@class": "apex.jorje.data.IndexLocation",
                          "startIndex": 30,
                          "endIndex": 31,
                          "line": 3,
                          "column": 10
                        },
                        "value": "c"
                      }
                    ],
                    "inputParameters": []
                  }
                }
              ]
            }
          }
        ],
        "superClass": {},
        "interfaces": []
      }
    },
    "hiddenTokenMap": []
  }
}

// a().b().c();
{
  "apex.jorje.semantic.compiler.parser.ParserOutput": {
    "internalErrors": [],
    "parseErrors": [],
    "unit": {
      "@class": "apex.jorje.data.ast.CompilationUnit$ClassDeclUnit",
      "body": {
        "@class": "apex.jorje.data.ast.ClassDecl",
        "loc": {
          "@class": "apex.jorje.data.IndexLocation",
          "startIndex": 0,
          "endIndex": 45,
          "line": 1,
          "column": 1
        },
        "modifiers": [],
        "name": {
          "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
          "loc": {
            "@class": "apex.jorje.data.IndexLocation",
            "startIndex": 6,
            "endIndex": 16,
            "line": 1,
            "column": 7
          },
          "value": "HelloWorld"
        },
        "typeArguments": {},
        "members": [
          {
            "@class": "apex.jorje.data.ast.BlockMember$StmntBlockMember",
            "stmnt": {
              "@class": "apex.jorje.data.ast.Stmnt$BlockStmnt",
              "loc": {
                "@class": "apex.jorje.data.IndexLocation",
                "startIndex": 21,
                "endIndex": 43,
                "line": 2,
                "column": 3
              },
              "stmnts": [
                {
                  "@class": "apex.jorje.data.ast.Stmnt$ExpressionStmnt",
                  "loc": {
                    "@class": "apex.jorje.data.IndexLocation",
                    "startIndex": 35,
                    "endIndex": 39,
                    "line": 3,
                    "column": 13
                  },
                  "expr": {
                    "@class": "apex.jorje.data.ast.Expr$MethodCallExpr",
                    "dottedExpr": {
                      "value": {
                        "@class": "apex.jorje.data.ast.Expr$MethodCallExpr",
                        "dottedExpr": {
                          "value": {
                            "@class": "apex.jorje.data.ast.Expr$MethodCallExpr",
                            "dottedExpr": {},
                            "isSafeNav": false,
                            "names": [
                              {
                                "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                                "loc": {
                                  "@class": "apex.jorje.data.IndexLocation",
                                  "startIndex": 27,
                                  "endIndex": 28,
                                  "line": 3,
                                  "column": 5
                                },
                                "value": "a"
                              }
                            ],
                            "inputParameters": []
                          }
                        },
                        "isSafeNav": false,
                        "names": [
                          {
                            "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                            "loc": {
                              "@class": "apex.jorje.data.IndexLocation",
                              "startIndex": 31,
                              "endIndex": 32,
                              "line": 3,
                              "column": 9
                            },
                            "value": "b"
                          }
                        ],
                        "inputParameters": []
                      }
                    },
                    "isSafeNav": false,
                    "names": [
                      {
                        "@class": "apex.jorje.data.Identifiers$LocationIdentifier",
                        "loc": {
                          "@class": "apex.jorje.data.IndexLocation",
                          "startIndex": 35,
                          "endIndex": 36,
                          "line": 3,
                          "column": 13
                        },
                        "value": "c"
                      }
                    ],
                    "inputParameters": []
                  }
                }
              ]
            }
          }
        ],
        "superClass": {},
        "interfaces": []
      }
    },
    "hiddenTokenMap": []
  }
}

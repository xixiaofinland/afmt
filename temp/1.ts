  const node = path.getNode();
  const nodeOp = getOperator(node);
  const nodePrecedence = getPrecedence(nodeOp);
  const parentNode = path.getParentNode();

  const isLeftNodeBinaryish = isBinaryish(node.left);
  const isRightNodeBinaryish = isBinaryish(node.right);
  const isNestedExpression = isBinaryish(parentNode);
  const isNestedRightExpression =
    isNestedExpression && node === parentNode.right;

  const isNodeSamePrecedenceAsLeftChild =
    isLeftNodeBinaryish &&
    nodePrecedence === getPrecedence(getOperator(node.left));
  const isNodeSamePrecedenceAsParent =
    isBinaryish(parentNode) &&
    nodePrecedence === getPrecedence(getOperator(parentNode));

  const docs: Doc[] = [];
  const leftDoc: Doc = path.call(print, "left");
  const operationDoc: Doc = path.call(print, "op");
  const rightDoc: Doc = path.call(print, "right");

  // This variable signifies that this node is a left child with the same
  // precedence as its parent, and thus should be laid out on the same indent
  // level as its parent, e.g:
  // a = b >
  //   c >  // -> the (b > c) node here
  //   d
  const isLeftChildNodeWithoutGrouping =
    (isNodeSamePrecedenceAsLeftChild || !isLeftNodeBinaryish) &&
    isNestedExpression &&
    isNodeSamePrecedenceAsParent &&
    !isNestedRightExpression;

  // #265 - This variable signifies that the right child of this node should
  // be laid out on the same indentation level, even though the left child node
  // should be in its own group, e.g:
  // a = b > c && d && e -> The node (d) here
  const hasRightChildNodeWithoutGrouping =
    !isLeftChildNodeWithoutGrouping &&
    isNestedExpression &&
    isNodeSamePrecedenceAsParent &&
    !isNestedRightExpression;

  // This variable signifies that the left node and right has the same
  // precedence, and thus they should be laid out on the same indent level, e.g.:
  // a = b > 1 &&
  //   c > 1
  const leftChildNodeSamePrecedenceAsRightChildNode =
    isLeftNodeBinaryish &&
    isRightNodeBinaryish &&
    getPrecedence(getOperator(node.left)) ===
      getPrecedence(getOperator(node.right));
  // This variable signifies that this node is the top most binaryish node,
  // and its left child node has the same precedence, e.g:
  // a = b >
  //   c >
  //   d  // -> the entire node (b > c > d) here
  const isTopMostParentNodeWithoutGrouping =
    isNodeSamePrecedenceAsLeftChild && !isNestedExpression;

  // If this expression is directly inside parentheses, we want to give it
  // an extra level indentation, i.e.:
  // ```
  // createObject(
  //   firstBoolean &&
  //      secondBoolean
  // );
  // ```
  // This is different behavior vs when the expression is in a variable
  // declaration, i.e.:
  // ```
  // firstBoolean =
  //   secondBoolean &&
  //   thirdBoolean;
  // ```
  // This behavior is consistent with how upstream formats Javascript
  const shouldIndentTopMostExpression = node.insideParenthesis;

  if (
    isLeftChildNodeWithoutGrouping ||
    leftChildNodeSamePrecedenceAsRightChildNode ||
    isTopMostParentNodeWithoutGrouping
  ) {
    docs.push(leftDoc);
    docs.push(" ");
    docs.push([operationDoc, line, rightDoc]);
    return shouldIndentTopMostExpression ? indentConcat(docs) : docs;
  }
  if (hasRightChildNodeWithoutGrouping) {
    docs.push(group(leftDoc));
    docs.push(" ");
    docs.push([operationDoc, line, rightDoc]);
    return docs;
  }
  // At this point we know that this node is not in a binaryish chain, so we
  // can safely group the left doc and right doc separately to have this effect:
  // a = b
  //  .c() > d
  docs.push(group(leftDoc));
  docs.push(" ");

  // If the left child of a binaryish expression has an end of line comment,
  // we want to make sure that comment is printed with the left child and
  // followed by a hardline. Otherwise, it will lead to unstable comments in
  // certain situation, because the EOL comment might become attached to the
  // entire binaryish expression after the first format.
  const leftChildHasEndOfLineComment =
    node.left.comments?.filter(
      (comment: AnnotatedComment) =>
        comment.trailing && comment.placement === "endOfLine",
    ).length > 0;

  if (leftChildHasEndOfLineComment) {
    docs.push(groupConcat([operationDoc, hardline, rightDoc]));
  } else {
    docs.push(groupConcat([operationDoc, line, rightDoc]));
  }
  return groupConcat(docs);


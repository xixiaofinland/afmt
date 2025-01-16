// step 1: npm install
// step 2: npx start-apex-server
// step 3: node jorje_ast.mjs test.cls

import fs from 'fs';
import path from 'path';

// Function to parse Apex code by sending it to the local server's /api/ast endpoint
async function parseTextWithHttp(
  text,
  serverHost = 'localhost',
  serverPort = 2117,
  serverProtocol = 'http',
  anonymous = false,
) {
  try {
    const response = await fetch(
      `${serverProtocol}://${serverHost}:${serverPort}/api/ast`,
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          sourceCode: text,
          anonymous,
          prettyPrint: false,
        }),
      }
    );

    const responseText = await response.text();
    console.log('Raw Response:', responseText);

    if (!response.ok) {
      throw new Error(
        `Server responded with status ${response.status}: ${responseText}`
      );
    }

    try {
      const ast = JSON.parse(responseText);
      return ast;
    } catch (jsonError) {
      throw new Error(`Failed to parse AST JSON: ${jsonError.message}`);
    }
  } catch (err) {
    throw new Error(
      `Failed to connect to Apex parsing server\r\n${err.toString()}`
    );
  }
}

async function main() {
  const filePath = process.argv[2];

  if (!filePath) {
    console.error('Usage: node printAst.mjs <path-to-file.cls>');
    process.exit(1);
  }

  const fullPath = path.resolve(filePath);

  if (!fs.existsSync(fullPath)) {
    console.error(`File not found: ${fullPath}`);
    process.exit(1);
  }

  // Read the file content
  const fileContent = fs.readFileSync(fullPath, 'utf8');
  console.log('File Content:\n', fileContent);

  try {
    const ast = await parseTextWithHttp(
      fileContent,
      'localhost',
      2117,
      'http',
      false
    );

    console.log('AST Output:\n', JSON.stringify(ast, null, 2));
  } catch (error) {
    console.error('Error generating AST:', error.message);
    process.exit(1);
  }
}

main();


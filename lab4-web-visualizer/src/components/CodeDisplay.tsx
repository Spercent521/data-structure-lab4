import React from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { atomDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import './CodeDisplay.css';

interface CodeDisplayProps {
  code: string;
}

const CodeDisplay: React.FC<CodeDisplayProps> = ({ code }) => {
  return (
    <div className="code-display">
      <SyntaxHighlighter
        language="rust"
        style={atomDark}
        showLineNumbers
        wrapLines={true}
        customStyle={{
          width: '100%',
          height: '100%',
          backgroundColor: '#1a1a1a',
          margin: 0,
        }}
        codeTagProps={{
          style: {
            fontFamily: 'monospace',
          },
        }}
      >
        {code}
      </SyntaxHighlighter>
    </div>
  );
};

export default CodeDisplay;

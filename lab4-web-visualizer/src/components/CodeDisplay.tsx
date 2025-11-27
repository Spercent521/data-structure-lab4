import React from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { atomDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import './CodeDisplay.css';

interface CodeDisplayProps {
  code: string;
  explanation?: string;
}

const CodeDisplay: React.FC<CodeDisplayProps> = ({ code, explanation }) => {
  return (
    <div className="code-display">
      <SyntaxHighlighter
        language="rust"
        style={atomDark}
        showLineNumbers
        wrapLines={true}
        customStyle={{
          width: '100%',
          height: 'calc(100% - 40px)',
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
      {explanation && <div className="explanation">{explanation}</div>}
    </div>
  );
};

export default CodeDisplay;

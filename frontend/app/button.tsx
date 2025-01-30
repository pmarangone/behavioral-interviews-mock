import React from 'react';
import { Copy } from 'lucide-react';

type CopyButtonProps = {
  onClick: () => void;
  className?: string;
};

export default function CopyButton({ onClick, className = '' }: CopyButtonProps) {
  return (
    <button
      onClick={onClick}
      className={`inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-gray-700 bg-gray-100 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 ${className}`}
      aria-label="Copy transcription"
    >
      <Copy className="w-4 h-4 mr-2" aria-hidden="true" />
      Copy
    </button>
  );
}
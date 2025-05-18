import React, { useState } from 'react';
import SignaturePad from './SignaturePad';

const SignDocument: React.FC = () => {
  const [signatureData, setSignatureData] = useState<string | null>(null);
  const [documentSigned, setDocumentSigned] = useState(false);
  
  const handleSignatureSave = (data: string) => {
    setSignatureData(data);
  };
  
  const handleDocumentSign = () => {
    if (signatureData) {
      setDocumentSigned(true);
      alert('문서가 성공적으로 서명되었습니다!');
    } else {
      alert('문서에 서명하기 전에 서명을 먼저 작성해주세요.');
    }
  };
  
  return (
    <div>
      <h1>문서 서명 페이지</h1>
      
      <div className="document-preview">
        <h2>샘플 문서</h2>
        <div className="document-content">
          <p>이 문서는 마이크로 프론트엔드 아키텍처 예시를 위한 샘플 문서입니다.</p>
          <p>아래에 서명을 작성하여 본 문서에 동의함을 표시해주세요.</p>
          <p>서명 후 "문서 서명" 버튼을 클릭하여 프로세스를 완료할 수 있습니다.</p>
          <hr />
          <p><strong>날짜:</strong> {new Date().toLocaleDateString()}</p>
        </div>
      </div>
      
      <SignaturePad onSave={handleSignatureSave} />
      
      <div className="document-actions">
        <button 
          onClick={handleDocumentSign} 
          disabled={!signatureData || documentSigned}
          className={documentSigned ? 'disabled' : 'save'}
        >
          {documentSigned ? '문서 서명 완료' : '문서 서명'}
        </button>
      </div>
      
      {documentSigned && (
        <div className="success-message">
          <h3>서명이 완료되었습니다!</h3>
          <p>문서가 성공적으로 처리되었습니다.</p>
        </div>
      )}
    </div>
  );
};

export default SignDocument; 
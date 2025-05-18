import React, { useRef, useState } from 'react';
import SignatureCanvas from 'react-signature-canvas';

type SignaturePadProps = {
  onSave?: (signatureData: string) => void;
};

const SignaturePad: React.FC<SignaturePadProps> = ({ onSave }) => {
  const sigPadRef = useRef<SignatureCanvas>(null);
  const [signatureImg, setSignatureImg] = useState<string | null>(null);

  const clear = () => {
    if (sigPadRef.current) {
      sigPadRef.current.clear();
      setSignatureImg(null);
    }
  };

  const save = () => {
    if (sigPadRef.current && !sigPadRef.current.isEmpty()) {
      const data = sigPadRef.current.toDataURL('image/png');
      setSignatureImg(data);
      if (onSave) {
        onSave(data);
      }
    }
  };

  return (
    <div>
      <h2>전자서명 패드</h2>
      <p>아래 영역에 서명을 그려주세요</p>
      
      <div className="sigpad-container">
        <SignatureCanvas
          ref={sigPadRef}
          penColor="black"
          canvasProps={{
            className: 'sigpad',
            width: 500,
            height: 200
          }}
        />
      </div>
      
      <div className="button-group">
        <button className="clear" onClick={clear}>서명 지우기</button>
        <button className="save" onClick={save}>서명 저장</button>
      </div>
      
      {signatureImg && (
        <div className="signature-preview">
          <h3>서명 미리보기</h3>
          <img src={signatureImg} alt="서명 이미지" />
        </div>
      )}
    </div>
  );
};

export default SignaturePad; 
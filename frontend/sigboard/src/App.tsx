import React from 'react';
import { Routes, Route } from 'react-router-dom';
import SignDocument from './components/SignDocument';

const App: React.FC = () => {
  return (
    <div className="sigboard-container">
      <Routes>
        <Route path="/" element={<SignDocument />} />
      </Routes>
    </div>
  );
};

export default App; 
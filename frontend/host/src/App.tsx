import React, { lazy, Suspense } from 'react';
import { Routes, Route, Link } from 'react-router-dom';

// 마이크로 프론트엔드 원격 모듈 동적 로딩
const SigboardApp = lazy(() => import('sigboard/SigboardApp'));
const AuthApp = lazy(() => import('auth/AuthApp'));

// 로딩 컴포넌트
const Loading = () => (
  <div className="loading">
    <p>로딩 중...</p>
  </div>
);

const App: React.FC = () => {
  return (
    <div className="container">
      <header>
        <h1>마이크로 프론트엔드 데모</h1>
        <nav>
          <ul>
            <li><Link to="/">홈</Link></li>
            <li><Link to="/sigboard">전자서명</Link></li>
            <li><Link to="/auth">로그인/회원가입</Link></li>
          </ul>
        </nav>
      </header>

      <main>
        <Suspense fallback={<Loading />}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/sigboard/*" element={<SigboardApp />} />
            <Route path="/auth/*" element={<AuthApp />} />
          </Routes>
        </Suspense>
      </main>

      <footer>
        <p>&copy; 2024 마이크로 프론트엔드 데모</p>
      </footer>
    </div>
  );
};

// 홈 컴포넌트
const Home: React.FC = () => (
  <div>
    <h2>마이크로 프론트엔드 아키텍처 데모</h2>
    <p>
      이 애플리케이션은 마이크로 프론트엔드 아키텍처를 시연합니다.
      전자서명 페이지는 React로, 인증 페이지는 Svelte로 구현되었습니다.
    </p>
  </div>
);

export default App; 
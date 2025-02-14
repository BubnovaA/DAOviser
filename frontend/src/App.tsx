import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import SpacesPage from './components/SpacesPage';
import ProposalsPage from './components/ProposalsPage';
import RecommendationPage from './components/RecommendationPage';

const App: React.FC = () => {
  return (
    <Router>
        <Routes>
          <Route path="/" element={<SpacesPage />} />
          <Route path="/proposals/:spaceId" element={<ProposalsPage />} />
          <Route path="/recommendation/:proposalId" element={<RecommendationPage />} />
        </Routes>
    </Router>
  );
};

export default App;
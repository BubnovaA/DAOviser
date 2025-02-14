/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { 
  Container, Typography, Button, CircularProgress, Alert, 
  Card, CardContent, Box, Divider 
} from '@mui/material';
import { Recommendation, fetchRecommendation } from '../api';
import FinalRecommendation from './FinalRecommendation';

const RecommendationPage: React.FC = () => {
  const { proposalId } = useParams<{ proposalId: string }>();
  const [recommendation, setRecommendation] = useState<Recommendation | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    let isMounted = true;  

    const loadRecommendation = async () => {
      setLoading(true);
      setError(null);
      
      try {
        const rec = await fetchRecommendation(proposalId!);
        
        if (isMounted) {
          if (rec && Object.keys(rec).length > 0) {
            setRecommendation(rec);
          } else {
            setError('No recommendation available for this proposal');
          }
        }
      } catch {
        if (isMounted) {
          setError('No recommendation available for this proposal');
        }
      } finally {
        if (isMounted) {
          setLoading(false);
        }
      }
    };

    if (proposalId) {
      loadRecommendation();
    }

    return () => {
      isMounted = false;  
    };
  }, [proposalId]);

  return (
    <Container sx={{ mt: 4, maxWidth: 800 }}>
      <Button variant="contained" onClick={() => navigate(-1)} sx={{ mb: 2 }}>
        Back
      </Button>

      {/* Заголовок */}
      <Typography
        variant="subtitle2"
        sx={{
          fontWeight: 700,
          color: 'text.secondary',
          mb: 1,
          textTransform: 'uppercase',
          fontSize: '1rem',
        }}
      >
        Voting Recommendation
      </Typography>
      <Divider sx={{ mb: 2 }} />

      {/* Лоадер */}
      {loading && (
        <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
          <CircularProgress />
        </Box>
      )}

      
      {!loading && error && (
        <Box sx={{ textAlign: 'center', mt: 4 }}>
          <Alert severity="info">{error}</Alert>
        </Box>
      )}

      
      {!loading && recommendation && (
        <Box>
          {[
            { title: 'Technical Impact', content: recommendation.technicalImpact },
            { title: 'Economic Consequences', content: recommendation.economicConsequences },
            { title: 'Governance & Decentralization', content: recommendation.governanceAndDecentralization },
          ].map(({ title, content }, index) => (
            <Card key={index} sx={{ mb: 2, p: 2 }}>
              <Typography variant="subtitle1" fontWeight={700} sx={{ fontSize: '1rem', mb: 1 }}>
                {title}
              </Typography>
              <Typography variant="body2" sx={{ fontSize: '0.875rem', color: 'text.secondary' }}>
                {content || 'No data available'}
              </Typography>
            </Card>
          ))}

          
          {recommendation.advantages && recommendation.advantages.length > 0 && (
            <Card sx={{ mb: 2, p: 2 }}>
              <Typography variant="subtitle1" fontWeight={700} sx={{ fontSize: '1rem', mb: 1 }}>
                Advantages
              </Typography>
              {recommendation.advantages.map((advantage: any, index: any) => (
                <Typography key={index} variant="body2" sx={{ fontSize: '0.875rem', color: 'text.secondary' }}>
                  • {advantage}
                </Typography>
              ))}
            </Card>
          )}

         
          {recommendation.risks && recommendation.risks.length > 0 && (
            <Card sx={{ mb: 2, p: 2 }}>
              <Typography variant="subtitle1" fontWeight={700} sx={{ fontSize: '1rem', mb: 1 }}>
                Risks
              </Typography>
              {recommendation.risks.map((risk: any, index: any) => (
                <Typography key={index} variant="body2" sx={{ fontSize: '0.875rem', color: 'text.secondary' }}>
                  • {risk}
                </Typography>
              ))}
            </Card>
          )}

          
          {recommendation.recommendation && Object.keys(recommendation.recommendation).length > 0 && (
            <Card sx={{ mb: 2, p: 3, textAlign: 'center' }}>
              <CardContent>
                <FinalRecommendation recommendation={recommendation.recommendation} />
              </CardContent>
            </Card>
          )}

         
          <Box sx={{ textAlign: 'center', mt: 3 }}>
            <Divider sx={{ mb: 2 }} />
            <Typography variant="body2" color="text.secondary">
              Created At: {recommendation.createdAt
                ? new Date(recommendation.createdAt * 1000).toLocaleString()
                : 'Unknown'}
            </Typography>
          </Box>
        </Box>
      )}
    </Container>
  );
};

export default RecommendationPage;

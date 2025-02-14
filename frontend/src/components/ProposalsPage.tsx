import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Container, Typography, CircularProgress, Alert, Paper, Divider, Box, Button, Link } from '@mui/material';
import { Proposal, fetchProposals } from '../api';

const ProposalsPage: React.FC = () => {
  const { spaceId } = useParams<{ spaceId: string }>();
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');
  const [expandedProposals, setExpandedProposals] = useState<{ [key: string]: boolean }>({});

  const navigate = useNavigate();

  useEffect(() => {
    if (spaceId) {
      const loadProposals = async () => {
        setLoading(true);
        try {
          const data = await fetchProposals(spaceId);
          setProposals(data);
        } catch (err) {
          setError((err as Error).message);
        } finally {
          setLoading(false);
        }
      };
      loadProposals();
    }
  }, [spaceId]);

  const toggleExpand = (proposalId: string) => {
    setExpandedProposals((prev) => ({
      ...prev,
      [proposalId]: !prev[proposalId],
    }));
  };

  return (
    <Container sx={{ mt: 4 }}>
      <Button variant="contained" onClick={() => navigate(-1)} sx={{ mb: 2 }}>
        Back to Spaces
      </Button>


       <Box
              sx={{
                borderBottom: '1px solid #eee',
                pb: 1,
                mb: 1,
              }}
            >
      <Typography
        variant="subtitle2"
        sx={{
          fontWeight: 700,
          color: 'text.secondary',
          mb: 1,
          textTransform: 'uppercase',
          fontSize: '1rem', // 16px
        }}
      >
        Proposals
      </Typography>
      <Divider sx={{ mb: 2 }} />
      </Box>

      {loading && <CircularProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      {proposals.length === 0 && !loading ? (
        <Typography>No proposals found.</Typography>
      ) : (
        <Box>
          {proposals.map((proposal) => {
            const isExpanded = expandedProposals[proposal.id] || false;
            const hasBody = typeof proposal.body === 'string' && proposal.body.trim().length > 0;
            const bodyPreview = hasBody && proposal.body && proposal.body.length > 150 ? proposal.body.slice(0, 150) + '...' : proposal.body;

            return (
              <Paper
                key={proposal.id}
                sx={{
                  p: 2,  
                  mb: 2,  
                  cursor: 'pointer',
                  transition: '0.2s ease-in-out',
                  '&:hover': { backgroundColor: 'action.hover' },
                }}
                onClick={() => navigate(`/recommendation/${proposal.id}`)}
              >
                
                <Typography
                  variant="h6"
                  sx={{
                    fontWeight: 700,
                    fontSize: '1rem', // 16px
                    mb: 0.5,
                  }}
                >
                  {proposal.title || 'No Title'}
                </Typography>

               
                {hasBody && (
                  <>
                    <Typography
                      variant="body2"
                      sx={{
                        fontSize: '0.875rem',  
                        color: 'text.secondary',
                        mb: 1,
                        display: 'block',
                      }}
                    >
                      {isExpanded ? proposal.body : bodyPreview}
                      {proposal.body && proposal.body.length > 150 && (
                        <Link
                          component="button"
                          onClick={(e) => {
                            e.stopPropagation();
                            toggleExpand(proposal.id);
                          }}
                          sx={{
                            ml: 1,
                            fontSize: '0.875rem',
                            fontWeight: 700,
                            cursor: 'pointer',
                          }}
                        >
                          {isExpanded ? 'Show less' : 'Read more'}
                        </Link>
                      )}
                    </Typography>
                  </>
                )}

             
                <Typography
                  variant="body2"
                  color="text.secondary"
                  sx={{ fontSize: '0.875rem' }}  
                >
                  Start: {proposal.start ? new Date(proposal.start * 1000).toLocaleString() : '-'}
                </Typography>
                <Typography
                  variant="body2"
                  color="text.secondary"
                  sx={{ fontSize: '0.875rem' }} 
                >
                  End: {proposal.end ? new Date(proposal.end * 1000).toLocaleString() : '-'}
                </Typography>
              </Paper>
            );
          })}
        </Box>
      )}
    </Container>
  );
};

export default ProposalsPage;

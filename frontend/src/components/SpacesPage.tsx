// src/components/SpacesPage.tsx

import React, { useState, useEffect } from 'react';
import {
  Container,
  Grid,
  Card,
  CardContent,
  Box,
  Typography,
  CircularProgress,
  Alert,
  Avatar,
  Divider,
} from '@mui/material';
import { useNavigate } from 'react-router-dom';
import { Space, fetchSpaces } from '../api';

 
const convertIpfsUrl = (url: string): string => {
  if (url.startsWith('ipfs://')) {
    return url.replace('ipfs://', 'https://ipfs.io/ipfs/');
  }
  return url;
};

const SpacesPage: React.FC = () => {
  const [spaces, setSpaces] = useState<Space[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');
  const navigate = useNavigate();

  useEffect(() => {
    const loadSpaces = async () => {
      setLoading(true);
      try {
        const data = await fetchSpaces();
        setSpaces(data);
      } catch (err) {
        setError((err as Error).message);
      } finally {
        setLoading(false);
      }
    };
    loadSpaces();
  }, []);

  return (
    <Container sx={{ mt: 4 }}>
 
      <Box
        sx={{
          borderBottom: '1px solid #eee',
          pb: 1,
          mb: 1,
        }}
      >
        <Typography
           sx={{
            fontWeight: 700,
            color: 'text.secondary',
            mb: 1,
            textTransform: 'uppercase',
            fontSize: '1rem',  
          }}
        >
          Spaces
        </Typography>
        <Divider sx={{ mb: 2 }} />
      </Box>

      {loading && <CircularProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      <Grid container spacing={3}>
        {spaces.map((space) => {
          const avatarUrl = convertIpfsUrl(space.space_avatar);

          return (
            <Grid item xs={12} sm={6} md={4} key={space.space_id}>
              <Card
                sx={{
                  cursor: 'pointer',
                  borderRadius: 2,
                  boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
                  transition: 'transform 0.2s ease',
                  '&:hover': {
                    transform: 'translateY(-2px)',
                    boxShadow: '0 4px 8px rgba(0,0,0,0.1)',
                  },
                }}
                onClick={() => navigate(`/proposals/${space.space_id}`)}
              >
               
                <Box
                  sx={{
                    background: 'linear-gradient(135deg, #ebebff 0%, #ffffff 100%)',
                    borderTopLeftRadius: 2,
                    borderTopRightRadius: 2,
                    padding: 2,
                  }}
                >
                  <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <Avatar
                      src={avatarUrl}
                      alt={space.space_name}
                      sx={{
                        width: 48,
                        height: 48,
                        mr: 1.5,
                        borderRadius: 2,
                        backgroundColor: '#fff',
                        border: '1px solid #e0e0e0',
                        objectFit: 'contain',
                      }}
                    />
                    <Typography variant="h6" sx={{ fontWeight: 600 }}>
                      {space.space_name}
                    </Typography>
                  </Box>
                </Box>

                
                <CardContent sx={{ pb: 2 }}>
                  <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <Typography component="span" variant="body2" sx={{ fontWeight: 700 }}>
                      proposals:
                    </Typography>
                    <Typography component="span" variant="body2" sx={{ ml: 0.5 }}>
                      {space.proposals_count}
                    </Typography>

                    <Box component="span" sx={{ mx: 2 }} />

                    <Typography component="span" variant="body2" sx={{ fontWeight: 700 }}>
                      active:
                    </Typography>
                    <Typography component="span" variant="body2" sx={{ ml: 0.5 }}>
                      {space.active_proposals_count}
                    </Typography>
                  </Box>
                </CardContent>
              </Card>
            </Grid>
          );
        })}
      </Grid>
    </Container>
  );
};

export default SpacesPage;

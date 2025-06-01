import React, { useState } from 'react';
import {
  Container,
  Typography,
  Box,
  Paper,
  TextField,
  Button,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Grid,
  Card,
  CardContent,
} from '@mui/material';
import { Api as ApiIcon, Send as SendIcon } from '@mui/icons-material';

const ApiTester: React.FC = () => {
  const [method, setMethod] = useState('GET');
  const [url, setUrl] = useState('/api/v1/products');
  const [headers, setHeaders] = useState('{"Content-Type": "application/json"}');
  const [body, setBody] = useState('');
  const [response, setResponse] = useState('');
  const [loading, setLoading] = useState(false);

  const handleSendRequest = async () => {
    setLoading(true);
    try {
      // Mock API response
      const mockResponse = {
        status: 200,
        data: {
          message: 'API request successful',
          timestamp: new Date().toISOString(),
          method: method,
          url: url,
        }
      };
      
      setTimeout(() => {
        setResponse(JSON.stringify(mockResponse, null, 2));
        setLoading(false);
      }, 1000);
    } catch (error) {
      setResponse(JSON.stringify({ error: 'Request failed' }, null, 2));
      setLoading(false);
    }
  };

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          <ApiIcon sx={{ mr: 2, verticalAlign: 'middle' }} />
          API Tester
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Test API endpoints and view responses
        </Typography>
      </Box>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              Request Configuration
            </Typography>
            
            <Grid container spacing={2}>
              <Grid item xs={12} sm={3}>
                <FormControl fullWidth>
                  <InputLabel>Method</InputLabel>
                  <Select
                    value={method}
                    label="Method"
                    onChange={(e) => setMethod(e.target.value)}
                  >
                    <MenuItem value="GET">GET</MenuItem>
                    <MenuItem value="POST">POST</MenuItem>
                    <MenuItem value="PUT">PUT</MenuItem>
                    <MenuItem value="DELETE">DELETE</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} sm={9}>
                <TextField
                  fullWidth
                  label="URL"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                />
              </Grid>
              
              <Grid item xs={12}>
                <TextField
                  fullWidth
                  label="Headers (JSON)"
                  multiline
                  rows={3}
                  value={headers}
                  onChange={(e) => setHeaders(e.target.value)}
                />
              </Grid>
              
              {(method === 'POST' || method === 'PUT') && (
                <Grid item xs={12}>
                  <TextField
                    fullWidth
                    label="Request Body (JSON)"
                    multiline
                    rows={4}
                    value={body}
                    onChange={(e) => setBody(e.target.value)}
                  />
                </Grid>
              )}
              
              <Grid item xs={12}>
                <Button
                  variant="contained"
                  startIcon={<SendIcon />}
                  onClick={handleSendRequest}
                  disabled={loading}
                  fullWidth
                >
                  {loading ? 'Sending...' : 'Send Request'}
                </Button>
              </Grid>
            </Grid>
          </Paper>
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              Response
            </Typography>
            <TextField
              fullWidth
              multiline
              rows={15}
              value={response}
              placeholder="Response will appear here..."
              InputProps={{
                readOnly: true,
                style: { fontFamily: 'monospace' }
              }}
            />
          </Paper>
        </Grid>

        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Quick Test Endpoints
              </Typography>
              <Grid container spacing={2}>
                <Grid item>
                  <Button
                    variant="outlined"
                    onClick={() => {
                      setMethod('GET');
                      setUrl('/api/v1/products');
                      setBody('');
                    }}
                  >
                    GET Products
                  </Button>
                </Grid>
                <Grid item>
                  <Button
                    variant="outlined"
                    onClick={() => {
                      setMethod('GET');
                      setUrl('/api/v1/users');
                      setBody('');
                    }}
                  >
                    GET Users
                  </Button>
                </Grid>
                <Grid item>
                  <Button
                    variant="outlined"
                    onClick={() => {
                      setMethod('POST');
                      setUrl('/api/v1/products');
                      setBody('{\n  "name": "New Product",\n  "price": 99.99,\n  "category": "Test"\n}');
                    }}
                  >
                    POST Product
                  </Button>
                </Grid>
                <Grid item>
                  <Button
                    variant="outlined"
                    onClick={() => {
                      setMethod('GET');
                      setUrl('/health');
                      setBody('');
                    }}
                  >
                    Health Check
                  </Button>
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Container>
  );
};

export default ApiTester; 
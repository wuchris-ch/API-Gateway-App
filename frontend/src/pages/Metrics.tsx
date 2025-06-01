import React, { useState, useEffect } from 'react';
import {
  Container,
  Typography,
  Box,
  Paper,
  Grid,
  Card,
  CardContent,
  LinearProgress,
} from '@mui/material';
import { Analytics as MetricsIcon } from '@mui/icons-material';

interface MetricsData {
  totalRequests: number;
  totalErrors: number;
  averageResponseTime: number;
  requestsPerSecond: number;
  errorRate: number;
}

const Metrics: React.FC = () => {
  const [metrics, setMetrics] = useState<MetricsData>({
    totalRequests: 0,
    totalErrors: 0,
    averageResponseTime: 0,
    requestsPerSecond: 0,
    errorRate: 0,
  });

  useEffect(() => {
    // Mock metrics data
    const mockMetrics: MetricsData = {
      totalRequests: 15420,
      totalErrors: 23,
      averageResponseTime: 125,
      requestsPerSecond: 45.2,
      errorRate: 0.15,
    };
    
    setTimeout(() => {
      setMetrics(mockMetrics);
    }, 500);
  }, []);

  const MetricCard: React.FC<{
    title: string;
    value: string | number;
    subtitle?: string;
    color?: string;
  }> = ({ title, value, subtitle, color = '#1976d2' }) => (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Typography variant="h6" component="div" gutterBottom>
          {title}
        </Typography>
        <Typography variant="h4" component="div" sx={{ color, mb: 1 }}>
          {value}
        </Typography>
        {subtitle && (
          <Typography variant="body2" color="text.secondary">
            {subtitle}
          </Typography>
        )}
      </CardContent>
    </Card>
  );

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          <MetricsIcon sx={{ mr: 2, verticalAlign: 'middle' }} />
          Metrics & Analytics
        </Typography>
        <Typography variant="body1" color="text.secondary">
          System performance and usage statistics
        </Typography>
      </Box>

      <Grid container spacing={3}>
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Total Requests"
            value={metrics.totalRequests.toLocaleString()}
            subtitle="All time"
            color="#1976d2"
          />
        </Grid>
        
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Total Errors"
            value={metrics.totalErrors}
            subtitle="Last 24 hours"
            color="#d32f2f"
          />
        </Grid>
        
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Avg Response Time"
            value={`${metrics.averageResponseTime}ms`}
            subtitle="Last hour"
            color="#ed6c02"
          />
        </Grid>
        
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Requests/sec"
            value={metrics.requestsPerSecond.toFixed(1)}
            subtitle="Current rate"
            color="#2e7d32"
          />
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              Error Rate
            </Typography>
            <Typography variant="h4" sx={{ mb: 2 }}>
              {metrics.errorRate.toFixed(2)}%
            </Typography>
            <LinearProgress
              variant="determinate"
              value={metrics.errorRate}
              color={metrics.errorRate < 1 ? 'success' : 'error'}
              sx={{ height: 10, borderRadius: 5 }}
            />
            <Typography variant="body2" color="text.secondary" sx={{ mt: 1 }}>
              Target: &lt; 1%
            </Typography>
          </Paper>
        </Grid>

        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              System Health
            </Typography>
            <Box sx={{ mb: 2 }}>
              <Typography variant="body2" gutterBottom>
                CPU Usage: 23%
              </Typography>
              <LinearProgress variant="determinate" value={23} sx={{ mb: 2 }} />
              
              <Typography variant="body2" gutterBottom>
                Memory Usage: 67%
              </Typography>
              <LinearProgress variant="determinate" value={67} color="warning" sx={{ mb: 2 }} />
              
              <Typography variant="body2" gutterBottom>
                Disk Usage: 45%
              </Typography>
              <LinearProgress variant="determinate" value={45} color="success" />
            </Box>
          </Paper>
        </Grid>

        <Grid item xs={12}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              Recent Activity Log
            </Typography>
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
              <Typography variant="body2" color="text.secondary">
                [2024-01-20 10:30:15] GET /api/v1/products - 200 OK (125ms)
              </Typography>
              <Typography variant="body2" color="text.secondary">
                [2024-01-20 10:30:12] POST /api/v1/orders - 201 Created (89ms)
              </Typography>
              <Typography variant="body2" color="text.secondary">
                [2024-01-20 10:30:08] GET /api/v1/users - 200 OK (45ms)
              </Typography>
              <Typography variant="body2" color="text.secondary">
                [2024-01-20 10:30:05] GET /health - 200 OK (12ms)
              </Typography>
              <Typography variant="body2" color="text.secondary">
                [2024-01-20 10:30:02] POST /auth/login - 200 OK (234ms)
              </Typography>
            </Box>
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default Metrics; 
import React, { useEffect, useState } from 'react';
import {
  Container,
  Grid,
  Paper,
  Typography,
  Box,
  Card,
  CardContent,
  LinearProgress,
  Chip,
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  TrendingUp,
  Speed,
  Security,
  CloudQueue,
} from '@mui/icons-material';

interface SystemStats {
  totalRequests: number;
  activeConnections: number;
  responseTime: number;
  uptime: string;
  errorRate: number;
}

const Dashboard: React.FC = () => {
  const [stats, setStats] = useState<SystemStats>({
    totalRequests: 0,
    activeConnections: 0,
    responseTime: 0,
    uptime: '0h 0m',
    errorRate: 0,
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate loading stats
    const timer = setTimeout(() => {
      setStats({
        totalRequests: 15420,
        activeConnections: 42,
        responseTime: 125,
        uptime: '2d 14h 32m',
        errorRate: 0.02,
      });
      setLoading(false);
    }, 1000);

    return () => clearTimeout(timer);
  }, []);

  const StatCard: React.FC<{
    title: string;
    value: string | number;
    icon: React.ReactNode;
    color: string;
    subtitle?: string;
  }> = ({ title, value, icon, color, subtitle }) => (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
          <Box sx={{ color, mr: 2 }}>{icon}</Box>
          <Typography variant="h6" component="div">
            {title}
          </Typography>
        </Box>
        <Typography variant="h4" component="div" sx={{ mb: 1 }}>
          {loading ? <LinearProgress /> : value}
        </Typography>
        {subtitle && (
          <Typography variant="body2" color="text.secondary">
            {subtitle}
          </Typography>
        )}
      </CardContent>
    </Card>
  );

  const ServiceStatus: React.FC<{
    name: string;
    status: 'healthy' | 'warning' | 'error';
    responseTime?: number;
  }> = ({ name, status, responseTime }) => {
    const getStatusColor = () => {
      switch (status) {
        case 'healthy': return 'success';
        case 'warning': return 'warning';
        case 'error': return 'error';
        default: return 'default';
      }
    };

    return (
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', py: 1 }}>
        <Typography variant="body1">{name}</Typography>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          {responseTime && (
            <Typography variant="body2" color="text.secondary">
              {responseTime}ms
            </Typography>
          )}
          <Chip
            label={status}
            color={getStatusColor()}
            size="small"
            variant="outlined"
          />
        </Box>
      </Box>
    );
  };

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          <DashboardIcon sx={{ mr: 2, verticalAlign: 'middle' }} />
          Dashboard
        </Typography>
        <Typography variant="body1" color="text.secondary">
          API Gateway system overview and monitoring
        </Typography>
      </Box>

      <Grid container spacing={3}>
        {/* Stats Cards */}
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Total Requests"
            value={stats.totalRequests.toLocaleString()}
            icon={<TrendingUp />}
            color="#1976d2"
            subtitle="Last 24 hours"
          />
        </Grid>
        
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Active Connections"
            value={stats.activeConnections}
            icon={<CloudQueue />}
            color="#2e7d32"
            subtitle="Current connections"
          />
        </Grid>
        
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Avg Response Time"
            value={`${stats.responseTime}ms`}
            icon={<Speed />}
            color="#ed6c02"
            subtitle="Last hour"
          />
        </Grid>
        
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="System Uptime"
            value={stats.uptime}
            icon={<Security />}
            color="#9c27b0"
            subtitle="Current session"
          />
        </Grid>

        {/* Service Status */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              Service Status
            </Typography>
            <ServiceStatus name="API Gateway" status="healthy" responseTime={45} />
            <ServiceStatus name="Backend API" status="healthy" responseTime={120} />
            <ServiceStatus name="Kong Gateway" status="healthy" responseTime={35} />
            <ServiceStatus name="PostgreSQL" status="healthy" responseTime={15} />
            <ServiceStatus name="Redis Cache" status="healthy" responseTime={8} />
            <ServiceStatus name="Keycloak" status="warning" responseTime={250} />
          </Paper>
        </Grid>

        {/* Recent Activity */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              Recent Activity
            </Typography>
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
              <Typography variant="body2" color="text.secondary">
                • New user registration: john.doe@example.com
              </Typography>
              <Typography variant="body2" color="text.secondary">
                • API key generated for service integration
              </Typography>
              <Typography variant="body2" color="text.secondary">
                • Rate limit threshold reached for client 192.168.1.100
              </Typography>
              <Typography variant="body2" color="text.secondary">
                • Health check passed for all backend services
              </Typography>
              <Typography variant="body2" color="text.secondary">
                • Configuration updated: new route added
              </Typography>
            </Box>
          </Paper>
        </Grid>

        {/* System Metrics */}
        <Grid item xs={12}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" gutterBottom>
              System Metrics
            </Typography>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={4}>
                <Typography variant="body2" color="text.secondary">
                  Error Rate
                </Typography>
                <Typography variant="h6">
                  {(stats.errorRate * 100).toFixed(2)}%
                </Typography>
                <LinearProgress 
                  variant="determinate" 
                  value={stats.errorRate * 100} 
                  color={stats.errorRate < 0.05 ? 'success' : 'error'}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <Typography variant="body2" color="text.secondary">
                  CPU Usage
                </Typography>
                <Typography variant="h6">23%</Typography>
                <LinearProgress variant="determinate" value={23} color="primary" />
              </Grid>
              <Grid item xs={12} sm={4}>
                <Typography variant="body2" color="text.secondary">
                  Memory Usage
                </Typography>
                <Typography variant="h6">67%</Typography>
                <LinearProgress variant="determinate" value={67} color="warning" />
              </Grid>
            </Grid>
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default Dashboard; 
import React, { useState, useEffect } from 'react';
import {
  Container,
  Typography,
  Box,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Chip,
} from '@mui/material';
import { People as UsersIcon } from '@mui/icons-material';

interface User {
  id: number;
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  is_active: boolean;
  is_admin: boolean;
}

const Users: React.FC = () => {
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    const mockUsers: User[] = [
      { id: 1, username: 'admin', email: 'admin@example.com', first_name: 'Admin', last_name: 'User', is_active: true, is_admin: true },
      { id: 2, username: 'john_doe', email: 'john@example.com', first_name: 'John', last_name: 'Doe', is_active: true, is_admin: false },
      { id: 3, username: 'jane_smith', email: 'jane@example.com', first_name: 'Jane', last_name: 'Smith', is_active: true, is_admin: false },
    ];
    setUsers(mockUsers);
  }, []);

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          <UsersIcon sx={{ mr: 2, verticalAlign: 'middle' }} />
          Users
        </Typography>
      </Box>

      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Username</TableCell>
              <TableCell>Email</TableCell>
              <TableCell>Name</TableCell>
              <TableCell>Status</TableCell>
              <TableCell>Role</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {users.map((user) => (
              <TableRow key={user.id}>
                <TableCell>{user.username}</TableCell>
                <TableCell>{user.email}</TableCell>
                <TableCell>{user.first_name} {user.last_name}</TableCell>
                <TableCell>
                  <Chip label={user.is_active ? 'Active' : 'Inactive'} color={user.is_active ? 'success' : 'default'} size="small" />
                </TableCell>
                <TableCell>
                  <Chip label={user.is_admin ? 'Admin' : 'User'} color={user.is_admin ? 'error' : 'primary'} size="small" />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </Container>
  );
};

export default Users; 
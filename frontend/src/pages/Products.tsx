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
  Button,
  Chip,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Grid,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Inventory as ProductsIcon,
} from '@mui/icons-material';

interface Product {
  id: number;
  name: string;
  description: string;
  price: number;
  category: string;
  stock_quantity: number;
  is_active: boolean;
}

const Products: React.FC = () => {
  const [products, setProducts] = useState<Product[]>([]);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingProduct, setEditingProduct] = useState<Product | null>(null);

  // Mock data
  useEffect(() => {
    const mockProducts: Product[] = [
      {
        id: 1,
        name: 'Laptop Pro',
        description: 'High-performance laptop for professionals',
        price: 1299.99,
        category: 'Electronics',
        stock_quantity: 50,
        is_active: true,
      },
      {
        id: 2,
        name: 'Wireless Headphones',
        description: 'Premium noise-cancelling headphones',
        price: 299.99,
        category: 'Electronics',
        stock_quantity: 100,
        is_active: true,
      },
      {
        id: 3,
        name: 'Coffee Maker',
        description: 'Automatic drip coffee maker',
        price: 89.99,
        category: 'Appliances',
        stock_quantity: 25,
        is_active: true,
      },
      {
        id: 4,
        name: 'Running Shoes',
        description: 'Comfortable running shoes for athletes',
        price: 129.99,
        category: 'Sports',
        stock_quantity: 75,
        is_active: true,
      },
      {
        id: 5,
        name: 'Smartphone',
        description: 'Latest generation smartphone',
        price: 899.99,
        category: 'Electronics',
        stock_quantity: 30,
        is_active: false,
      },
    ];

    setTimeout(() => {
      setProducts(mockProducts);
    }, 500);
  }, []);

  const handleAddProduct = () => {
    setEditingProduct(null);
    setDialogOpen(true);
  };

  const handleEditProduct = (product: Product) => {
    setEditingProduct(product);
    setDialogOpen(true);
  };

  const handleDeleteProduct = (id: number) => {
    setProducts(products.filter(p => p.id !== id));
  };

  const handleCloseDialog = () => {
    setDialogOpen(false);
    setEditingProduct(null);
  };

  const ProductDialog: React.FC = () => {
    const [formData, setFormData] = useState({
      name: editingProduct?.name || '',
      description: editingProduct?.description || '',
      price: editingProduct?.price || 0,
      category: editingProduct?.category || '',
      stock_quantity: editingProduct?.stock_quantity || 0,
    });

    const handleSubmit = () => {
      if (editingProduct) {
        // Update existing product
        setProducts(products.map(p => 
          p.id === editingProduct.id 
            ? { ...p, ...formData }
            : p
        ));
      } else {
        // Add new product
        const newProduct: Product = {
          id: Math.max(...products.map(p => p.id)) + 1,
          ...formData,
          is_active: true,
        };
        setProducts([...products, newProduct]);
      }
      handleCloseDialog();
    };

    return (
      <Dialog open={dialogOpen} onClose={handleCloseDialog} maxWidth="md" fullWidth>
        <DialogTitle>
          {editingProduct ? 'Edit Product' : 'Add New Product'}
        </DialogTitle>
        <DialogContent>
          <Grid container spacing={2} sx={{ mt: 1 }}>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Product Name"
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Category"
                value={formData.category}
                onChange={(e) => setFormData({ ...formData, category: e.target.value })}
              />
            </Grid>
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Description"
                multiline
                rows={3}
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Price"
                type="number"
                value={formData.price}
                onChange={(e) => setFormData({ ...formData, price: parseFloat(e.target.value) })}
              />
            </Grid>
            <Grid item xs={12} sm={6}>
              <TextField
                fullWidth
                label="Stock Quantity"
                type="number"
                value={formData.stock_quantity}
                onChange={(e) => setFormData({ ...formData, stock_quantity: parseInt(e.target.value) })}
              />
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancel</Button>
          <Button onClick={handleSubmit} variant="contained">
            {editingProduct ? 'Update' : 'Add'}
          </Button>
        </DialogActions>
      </Dialog>
    );
  };

  return (
    <Container maxWidth="lg">
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          <ProductsIcon sx={{ mr: 2, verticalAlign: 'middle' }} />
          Products
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Manage your product catalog
        </Typography>
      </Box>

      <Box sx={{ mb: 3 }}>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={handleAddProduct}
        >
          Add Product
        </Button>
      </Box>

      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Category</TableCell>
              <TableCell>Price</TableCell>
              <TableCell>Stock</TableCell>
              <TableCell>Status</TableCell>
              <TableCell>Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {products.map((product) => (
              <TableRow key={product.id}>
                <TableCell>
                  <Box>
                    <Typography variant="body1" fontWeight="medium">
                      {product.name}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      {product.description}
                    </Typography>
                  </Box>
                </TableCell>
                <TableCell>{product.category}</TableCell>
                <TableCell>${product.price.toFixed(2)}</TableCell>
                <TableCell>{product.stock_quantity}</TableCell>
                <TableCell>
                  <Chip
                    label={product.is_active ? 'Active' : 'Inactive'}
                    color={product.is_active ? 'success' : 'default'}
                    size="small"
                  />
                </TableCell>
                <TableCell>
                  <IconButton
                    size="small"
                    onClick={() => handleEditProduct(product)}
                    color="primary"
                  >
                    <EditIcon />
                  </IconButton>
                  <IconButton
                    size="small"
                    onClick={() => handleDeleteProduct(product.id)}
                    color="error"
                  >
                    <DeleteIcon />
                  </IconButton>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

      <ProductDialog />
    </Container>
  );
};

export default Products; 
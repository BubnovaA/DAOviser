import React from 'react';
import { PieChart, Pie, Cell, Tooltip, ResponsiveContainer } from 'recharts';
import { useTheme } from '@mui/material/styles';

type FinalRecommendationProps = {
  recommendation: Record<string, number>;
};

const FinalRecommendation: React.FC<FinalRecommendationProps> = ({ recommendation }) => {
  const theme = useTheme();  
  const primaryColor = theme.palette.primary.main;  
  const competitorColor = '#FD7B7C';  

  const data = Object.entries(recommendation).map(([option, value]) => ({
    name: option,
    value: value * 100, 
  }));

   
  const sortedData = [...data].sort((a, b) => b.value - a.value);


  const COLORS = [
    primaryColor,     
    competitorColor,  
    '#388E3C',      
    '#4CAF50',      
    '#81C784',      
  ];

  return (
    <ResponsiveContainer width="100%" height={250}>
      <PieChart>
        <Pie
          data={sortedData}
          cx="50%"
          cy="50%"
          innerRadius={60}
          outerRadius={85}
          paddingAngle={4}
          dataKey="value"
          label={({ name, percent }) => `${name} ${(percent * 100).toFixed(0)}%`}
        >
          {sortedData.map((_, index) => (
            <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
          ))}
        </Pie>
        <Tooltip />
      </PieChart>
    </ResponsiveContainer>
  );
};

export default FinalRecommendation;

import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import Login from './Login';
import LoginCallback from './LoginCallback';
import AuthProvider from './auth/AuthProvider';
import ProtectedRoutes from './auth/ProtectedRoutes';

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);

root.render(
    <React.StrictMode>
        <AuthProvider>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={
                        <ProtectedRoutes>
                            <App />
                        </ProtectedRoutes>
                    } />
                    <Route path="/login" element={<Login />} />
                    <Route path="/login-callback" element={<LoginCallback />} />
                </Routes>
            </BrowserRouter>
        </AuthProvider>
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
console.log(reportWebVitals());

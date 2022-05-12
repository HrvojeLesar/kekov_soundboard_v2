import { useContext } from 'react';
import logo from './logo.svg';
import './App.css';
import { Outlet, Route, Routes, useSearchParams } from 'react-router-dom';
import { Link } from 'react-router-dom';
import { AuthContext } from './auth/AuthProvider';
import { AppShell } from '@mantine/core';
import Sidebar from './components/Sidebar';
import { Guild } from './components/Guild';

function App() {
    let [searchParams, setSearchParams] = useSearchParams();
    let { user, logout } = useContext(AuthContext);

    return (
        <div className="App">
            <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <img src={user?.avatar ? `https://cdn.discordapp.com/avatars/${user.id}/${user.avatar}.png` : `https://cdn.discordapp.com/embed/avatars/1.png`} alt="logo" />
                {<button onClick={() => { console.log(searchParams.values().next()) }}>Print params</button>}
                {<button onClick={() => { logout() }}>Logout</button>}
                <p>
                    Edit <code>src/App.tsx</code> brofist.
                </p>
                <a href="http://localhost:8080/v1/auth/init">LOGIN</a>
                <Link to="login-callback">Login Callback</Link>
                <a
                    className="App-link"
                    href="https://reactjs.org"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    Learn React
                </a>
            </header>
        </div>
    );
}

export default App;

import {render} from 'react-dom';
import {BrowserRouter, Route, Routes} from 'react-router-dom';
import {withStore} from 'react-context-hook';
import './index.css';

import {Home,} from './components';
import {AppBar} from './components/AppBar';

const initialValue: { [id: string]: any } = {
    'login-widget': null,
    'login-open': false,
    user: null,
};

function AppX() {
    return (
        <BrowserRouter>
            <AppBar/>
            <Routes>
                <Route path="/" element={<Home/>}/>
            </Routes>
        </BrowserRouter>
    );
}

const App = withStore(AppX, initialValue);

render(<App/>, document.getElementById('root'));

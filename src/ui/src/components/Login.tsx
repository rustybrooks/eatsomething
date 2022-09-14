import { useCallback, useState } from 'react';

import { useGetAndSet } from 'react-context-hook';

import * as constants from '../constants';
import { Button } from './widgets/Button';
import { TextInput } from './widgets/TextInput';
import { Box } from './widgets/Box';
import { Tabs } from './widgets/Tabs';

const style = {
  root: {
    maxWidth: 600,
    minWidth: 400,
    // float: 'left',
  },

  formControl: {
    margin: '10px',
    minWidth: 120,
  },

  button: {
    margin: '10px',
  },
};

const genUrl = (fn = '') => `${constants.BASE_URL}/api/user/${fn}`;

export function Login({ updateUser }: { updateUser: any }) {
  const [tab, setTab] = useState('login');
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [password2, setPassword2] = useState('');
  const [errors, setErrors]: [{ username?: string; email?: string; password?: string; password2?: string }, any] = useState({});

  const [, setLoginOpen] = useGetAndSet('login-open');

  function closeDrawer() {
    setLoginOpen(false);
  }

  const doCancel = useCallback(() => {
    closeDrawer();
  }, []);

  const doLogin = useCallback(async () => {
    const result = await fetch(genUrl('login'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        password,
      }),
    });
    if (result.status === 403) {
      localStorage.setItem('api-key', null);
      setErrors({ ...errors, password: 'Error logging in' });
    } else {
      setErrors({});
      const foo: any = await result.json()
      localStorage.setItem('api-key', foo.token);
      closeDrawer();
      updateUser();
    }
  }, [username, password]);

  const doSignup = async () => {
    const result = await fetch(genUrl('signup'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username,
        email,
        password,
        password2,
      }),
    });
    if (result.status === 400) {
      setErrors((await result.json()).details);
    } else {
      setErrors({});
      const foo: any = await result.json()
      localStorage.setItem('api-key', await foo.token);
      closeDrawer();
      updateUser();
    }
  };

  return (
    <div style={style.root}>
      <Tabs
        value={tab}
        tabs={[
          ['Login', 'login'],
          ['Signup', 'signup'],
        ]}
        onChange={(e: string) => {
          setErrors({});
          setTab(e);
        }}
      />

      <Box style={{ display: tab === 'signup' ? 'block' : 'none', textAlign: 'center' }}>
        <TextInput
          style={{ width: '90%' }}
          error={Boolean(errors.username)}
          helperText={errors.username}
          label="Username"
          value={username}
          autoFocus={true}
          onChange={(event: any) => setUsername(event.target.value)}
        />

        <TextInput
          style={{ width: '90%' }}
          error={Boolean(errors.email)}
          helperText={errors.email}
          label="Email"
          value={email}
          onChange={(event: any) => setEmail(event.target.value)}
        />

        <TextInput
          style={{ width: '90%' }}
          error={Boolean(errors.password)}
          helperText={errors.password}
          label="Password"
          value={password}
          type="password"
          onChange={(event: any) => setPassword(event.target.value)}
        />

        <TextInput
          style={{ width: '90%' }}
          error={Boolean(errors.password2)}
          helperText={errors.password2}
          label="Confirm Password"
          value={password2}
          type="password"
          onChange={(event: any) => setPassword2(event.target.value)}
        />

        <Button style={style.button} onClick={doCancel} variant="contained">
          Cancel
        </Button>
        <Button style={style.button} onClick={doSignup} variant="contained" color="blue">
          Sign up
        </Button>
      </Box>

      <Box style={{ display: tab === 'login' ? 'block' : 'none', textAlign: 'center' }}>
        <TextInput
          style={{ width: '90%', margin: '.2rem' }}
          error={Boolean(errors.username)}
          helperText={errors.username}
          label="Username"
          value={username}
          onChange={(event: any) => setUsername(event.target.value)}
          autoFocus={true}
        />

        <TextInput
          style={{ width: '90%', margin: '.2rem' }}
          error={Boolean(errors.password)}
          helperText={errors.password}
          label="Password"
          value={password}
          type="password"
          onChange={(event: any) => setPassword(event.target.value)}
          onKeyPress={(event: any) => (event.key.toLowerCase() === 'enter' ? doLogin() : null)}
        />

        <Button style={style.button} onClick={doCancel} variant="contained">
          Cancel
        </Button>
        <Button style={style.button} onClick={doLogin} variant="contained" color="blue">
          Login
        </Button>
      </Box>
    </div>
  );
}

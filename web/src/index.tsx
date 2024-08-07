/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'

import { Demo } from './Demo'
import { initializeWasm } from '@lib/main';

////////////////////////////////////////////////////////////////////////////////

await initializeWasm();

const root = document.getElementById('root');
render(() => <Demo />, root!);

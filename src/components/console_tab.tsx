import { useContext, useState } from 'react';
import { listen } from "@tauri-apps/api/event";
import Button from 'react-bootstrap/Button';
import Modal from 'react-bootstrap/Modal';
import Table from 'react-bootstrap/Table';

import { toast } from 'react-toastify';

import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

import { store } from './../store.js';

import { ConsoleEvent } from '../interface/console_event';

interface ConsoleEventMessage {
  payload: ConsoleEvent;
}

function ConsoleTab() {
  const [consoleText, setConsoleText] = useState("");

  const { state, dispatch } = useContext(store);

  return (
    <div>
      <pre>{state.console}</pre>
    </div>
  );
}

export default ConsoleTab;
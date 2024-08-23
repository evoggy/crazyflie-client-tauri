import { useEffect, useContext } from 'react';
import { listen } from "@tauri-apps/api/event";
import Button from 'react-bootstrap/Button';
import Modal from 'react-bootstrap/Modal';
import Table from 'react-bootstrap/Table';

import { toast } from 'react-toastify';

import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

import { store } from './store.js';

import { ConsoleEvent } from './interface/console_event';

interface ConsoleEventMessage {
  payload: ConsoleEvent;
}

function BackEnd() {
  const { state, dispatch } = useContext(store);

  const changeColor = () => {
    dispatch({ type: "CHANGE_COLOR", payload: "blue" });
  };

  useEffect(() => {
    console.log("Backend loaded");
    const unListen = listen("console", (e: ConsoleEventMessage) => {
      //setConsoleText(consoleText + e.payload.message + "\n");
      console.log("Got console stuff off backend")
      dispatch({ type: "CONSOLE", payload: e.payload.message });

    });

    return () => {
      console.log("Backend unloaded");
      unListen.then((f) => f());
    };
  }, []);


  return null;
}

export default BackEnd;
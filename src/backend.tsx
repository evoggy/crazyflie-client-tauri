import { useEffect } from 'react';
import { listen } from "@tauri-apps/api/event";

import { ConsoleEvent } from './interface/console_event';

import {useStore} from "./App";

interface ConsoleEventMessage {
  payload: ConsoleEvent;
}

function BackEnd() {
  const [_, setStore] = useStore((store) => store["console"]);

  let currentConsoleText = "";

  useEffect(() => {
    console.log("Backend loaded");
    
    const unListen = listen("console", (e: ConsoleEventMessage) => {
      currentConsoleText += e.payload.message + "\n";
      setStore({ ["console"]: currentConsoleText });
    });

    return () => {
      console.log("Backend unloaded");
      unListen.then((f) => f());
    };
  }, []);


  return null;
}

export default BackEnd;
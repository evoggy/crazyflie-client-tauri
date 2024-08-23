import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import { Routes, Route, Outlet, Link } from "react-router-dom";
import { Sidebar, Menu, MenuItem, SubMenu } from 'react-pro-sidebar';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  console.log("Rendering main app")

  return (
    <div className="container" style={{ display: 'flex', height: '100%' }}>
      <Sidebar
        //backgroundColor="#f6f6f6"
        breakPoint="md"
      >

        <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
          

          <Menu>
            <SubMenu label="Charts">
              <MenuItem> Pie charts </MenuItem>
              <MenuItem> Line charts </MenuItem>
            </SubMenu>
            <MenuItem> Documentation </MenuItem>
            <MenuItem> Calendar </MenuItem>
          </Menu>
        </div>
      </Sidebar>
    </div>
  );
}

export default App;

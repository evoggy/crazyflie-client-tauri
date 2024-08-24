import { useState } from 'react';
import "./App.css";

import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

import { Routes, Route, Outlet, Link } from "react-router-dom";
import { Sidebar, Menu, MenuItem, SubMenu } from 'react-pro-sidebar';

import 'bootstrap/dist/css/bootstrap.min.css';

import ConnectionModal from "./components/connection_modal";
import ConsoleTab from "./components/console_tab";
import BackEnd from './backend';


import createFastContext from "./global_context";

export const { Provider, useStore } = createFastContext({
  console: ""
});


function App() {

  return (
    <Provider>
      <div>
      {/* Routes nest inside one another. Nested route paths build upon
            parent route paths, and nested route elements render inside
            parent route elements. See the note about <Outlet> below. */}
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="fly" element={<Fly />} />
          <Route path="console" element={<ConsoleTab />} />

          {/* Using path="*"" means "match anything", so this route
                acts like a catch-all for URLs that we don't have explicit
                routes for. */}
          <Route path="*" element={<NoMatch />} />
        </Route>
      </Routes>
      <BackEnd/>
      </div>
    </Provider>

  );
}

function Layout() {
  const [show, setShow] = useState(false);

  const handleClose = () => setShow(false);
  const handleShow = () => setShow(true);
  const test_error_toast = () => toast.error("Oooh noos! An error!");

  return (
    <div>

      <Sidebar width={"200"}>
        <Menu>
          <MenuItem onClick={handleShow}>Connect</MenuItem>

          <MenuItem component={<Link to="/fly" />}>Flight</MenuItem>
          <MenuItem component={<Link to="/console" />}>Console</MenuItem>
          <SubMenu label="Logs">
            <MenuItem>TOC</MenuItem>
            <MenuItem>Plotting</MenuItem>
          </SubMenu>
          <SubMenu label="Parameters">
            <MenuItem>List</MenuItem>
          </SubMenu>
          <SubMenu label="Toast tests">
            <MenuItem onClick={test_error_toast}>Error</MenuItem>
          </SubMenu>
        </Menu>
      </Sidebar>


      <div className="main-app-window">
        <Container fluid>
          <Row>
            <Col>
              <Outlet />
            </Col>
          </Row>
        </Container>
      </div>

      <div>
        <ConnectionModal handleClose={handleClose} shouldShow={show} />
      </div>

      <div>
        <ToastContainer
          position="bottom-center"
          autoClose={5000}
          hideProgressBar={false}
          newestOnTop={false}
          closeOnClick
          rtl={false}
          pauseOnFocusLoss
          draggable
          pauseOnHover
          theme="light"
        />
      </div>

    </div>

  );
}

function Home() {
  return (
    <div>
      <h2>Home</h2>
    </div>
  );
}

function Fly() {
  return (
    <div>
      <h2>Fly</h2>
    </div>
  );
}

function NoMatch() {
  return (
    <div>
      <h2>Nothing to see here!</h2>
      <p>
        <Link to="/">Go to the home page</Link>
      </p>
    </div>
  );
}

export default App;

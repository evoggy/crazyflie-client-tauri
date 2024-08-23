import { useEffect, useState } from 'react';
import { listen } from "@tauri-apps/api/event";
import Button from 'react-bootstrap/Button';
import Modal from 'react-bootstrap/Modal';
import Table from 'react-bootstrap/Table';

import { toast } from 'react-toastify';

import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

import 'bootstrap/dist/css/bootstrap.min.css';

import { invoke } from "@tauri-apps/api/tauri";

import { ScanResponse } from '../interface/scan_response';


// interface ProgressEventPayload {
//   matches_per_bs: Stats;
//   hits_per_sensor: Stats;
//   sync: Stats;
// }

interface ProgressEventProps {
  payload: ScanResponse;
}

function ConnectionModal({ handleClose, shouldShow }: { handleClose: any, shouldShow: boolean }) {
  const [crazyflies, setCrazyflies] = useState<string[]>([]);

  useEffect(() => {
    console.log("Reception tab loaded");
    const unListen = listen("scan", (e: ProgressEventProps) => {
      if (e.payload.err !== null) {
        toast.error(e.payload.err)
        handleClose()
      } else {
        setCrazyflies(e.payload.uris)
      }

    });

    return () => {
      console.log("Reception tab unloaded");
      unListen.then((f) => f());
    };
  }, []);

  async function scan() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    try {
      // let cfs: string[] = await invoke("scan", { address: "something" });
      // console.log(cfs)
      // setCrazyflies(cfs);
      await invoke("start_scan", { address: "something" });
    } catch (e) {
      console.log("Error right here!" + e)
      toast.error("" + e)
      handleClose()
    }

  }

  async function stop_scan() {
    await invoke("stop_scan", {});
    setCrazyflies([]);
    handleClose()
  }
  
  async function connect(uri: string) {
    await invoke("stop_scan", {});
    await invoke("connect", { uri: uri });
    handleClose()
  }

  

  return (

    <div>
      <Modal show={shouldShow} onHide={stop_scan} onShow={scan} animation={false}>
        <Modal.Header closeButton>
          <Modal.Title>Select Crazyflie ...</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Table borderless>
            <tbody>
              {crazyflies.map((uri, index) => (
                <tr key={index}>
                  <td>{index}</td>
                  <td>{uri}</td>
                  <td>
                    <Button variant="primary" onClick={() => connect(uri)}>
                      Connect
                    </Button>
                  </td>
                  <td>
                    <Button disabled variant="primary">
                      Bootload
                    </Button>
                  </td>                  
                </tr>
              ))}
            </tbody>
          </Table>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
            Cancel
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
}

export default ConnectionModal;

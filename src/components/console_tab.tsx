import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import {useStore} from "./../App";

function ConsoleTab() {

  const [crazyflieConsole] = useStore((store) => store["console"]);

  return (
    <div>
      <Row>
        <Col>
          <pre>{crazyflieConsole}</pre>
        </Col>
      </Row>
    </div>
  );
}

export default ConsoleTab;
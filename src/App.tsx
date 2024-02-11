import { useEffect } from "react";
import { commands } from "./bindings";

function App() {
  useEffect(() => {
    commands.closeSplashscreen();
  }, []);

  return <h1>Test</h1>;
}

export default App;

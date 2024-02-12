import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    invoke("close_splashscreen")
  }, []);

  return <h1>Test</h1>;
}

export default App;

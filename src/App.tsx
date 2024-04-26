import { invoke } from '@tauri-apps/api'
import { useState } from 'react'

function App() {
  const [data, setData] = useState<string>("")
  const parse_docker_compose = async () => {
    const response: string = await invoke('parse_docker_compose', { filePath: 'src/docker-compose.yml' })
    setData(response)
    console.log('response ', response)
  }
  return (
    <>
      <button onClick={parse_docker_compose}></button>
      <p>{data}</p>
    </>
  )
}

export default App

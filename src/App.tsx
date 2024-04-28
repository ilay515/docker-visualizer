import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from "react"
import "./App.css"
import Graph, { GraphProps } from './components/graph/graph'
import { Card, Flex, Layout } from 'antd'

import { Content, Header } from 'antd/es/layout/layout'
import Sider from 'antd/es/layout/Sider'
import Title from 'antd/es/typography/Title'

function App() {
  const [graph, setGraph] = useState<GraphProps | undefined>(undefined)

  useEffect(() => {
    const getGraph = async () => {
      const response: string = await invoke('parse_docker_compose', { filePath: 'src/docker-compose.yml' })
      let graph = JSON.parse(response)
      console.log(graph)
      setGraph(graph)
    }
    getGraph().catch(console.error)
  }, [])

  return (
    <div className="App" >
      <Layout className='page-layout' style={{ minHeight: '100vh' }}>
        <Header>
          <Flex justify='center'>
            <Title style={{ color: 'white', justifyContent: 'center' }}>
              Docker Visualizer
            </Title>
          </Flex>
        </Header>
        <Content
          style={{
            margin: '24px 16px',
            padding: 24,
            minHeight: 280,
          }}
        >
          <div style={{ width: '100%', height: 'calc(100vh - 124px)', position: 'relative', backgroundColor: "#1f1f1f" }}>
            {graph ? <Graph {...graph} /> : <p>YO</p>}
          </div>
        </Content>
      </Layout>
    </div>
  )
}

export default App

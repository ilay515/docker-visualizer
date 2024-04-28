import { Card } from "antd"

export interface ServiceProps {
  name: string,
  image: String,
  ports: String[]
}

function Service({ name, image, ports }: ServiceProps) {
  return (
    <div className="service">
      <Card title="Card title" bordered={false} style={{ width: 300 }}>
        <p>{name}</p>
        <p>{image}</p>
        <p>{ports}</p>
      </Card>
    </div>
  )
}

export default Service



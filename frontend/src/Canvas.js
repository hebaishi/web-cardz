import React, { useState } from 'react';
import { Stage, Layer, Text, Image } from 'react-konva';
import Konva from 'konva';
import useImage from 'use-image';

const ColoredRect = () => {
  const [image] = useImage('public/images/aceofspades.svg');
  const scale = 0.25;
  const [position, setPosition] = useState({
    isDragging: false,
    x: 50,
    y: 50,
  });

  const handleClick = () => {
    setColor(Konva.Util.getRandomColor());
  };

  return (
    <Layer>
      <Text
        text={"x:" + String(position.x) + "y:" + String(position.y)}
      ></Text>
      <Image
        scaleX={scale}
        scaleY={scale}
        image={image}
        x={20}
        y={20}
        draggable
        onDragStart={() => {
          setPosition({
            isDragging: true,
          });
        }}
        onDragEnd={(e) => {
          setPosition({
            isDragging: false,
            x: e.target.x(),
            y: e.target.y(),
          });
        }}
        onClick={handleClick}
      />
    </Layer>
  );
};

const Canvas = () => {
  return (
    <Stage width={window.innerWidth} height={window.innerHeight}>
      <ColoredRect />
    </Stage>
  );
};

export default Canvas;

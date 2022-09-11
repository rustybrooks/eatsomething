import { forwardRef } from 'react';
import './TitleBox.css';

interface PropsTypes {
  children: any;
  width?: string;
  title: string;
  style?: any;
}

export const TitleBox = forwardRef((props: PropsTypes, ref: any) => {
  return (
    <div className="titlebox" tabIndex={-1} style={{ width: props.width, ...props.style }} ref={ref}>
      <div className="titlebox-title">{props.title}</div>
      <div className="titlebox-content">{props.children}</div>
    </div>
  );
});

import { forwardRef } from 'react';
import './Drawer.css';

interface drawerProps {
  children: any;
  onClose: any;
  open: boolean;
  anchor: 'bottom' | 'right';
  style?: any;
}

export const Drawer = forwardRef((props: drawerProps, ref: any) => {
  const openStyle = {};

  const closedStyle = {
    visibility: 'hidden',
  };

  return (
    <div className="drawer-parent" style={{ ...props.style, ...(props.open ? openStyle : closedStyle) }} ref={ref}>
      <div className="drawer-background" onClick={props.onClose} />
      <div tabIndex={0} data-test="sentinelStart" />
      <div className={`drawerbox ${props.anchor}`} tabIndex={-1}>
        {props.children}
      </div>
    </div>
  );
});

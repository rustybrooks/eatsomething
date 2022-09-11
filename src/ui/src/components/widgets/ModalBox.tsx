import { forwardRef } from 'react';
import './ModalBox.css';

interface ModalProps {
  children: any;
  width: string;
  onClose: any;
  open: boolean;
}

export const ModalBox = forwardRef((props: ModalProps, ref: any) => {
  const openStyle = {};

  const closedStyle = {
    visibility: 'hidden',
  };

  return (
    <div className="modal-parent" style={props.open ? openStyle : closedStyle} ref={ref}>
      <div className="modal-background" onClick={props.onClose} />
      <div className="modal-sentinel" />
      <div tabIndex={0} data-test="sentinelStart" />
      <div className="modalbox" tabIndex={-1} style={{ width: props.width }}>
        {props.children}
      </div>
    </div>
  );
});

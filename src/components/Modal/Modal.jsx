import React from 'react';
import reactDom from 'react-dom';
import './Modal.css'

const Modal = ({ open, children, onClose, buttons }) => {
    if (!open) return null;
    return reactDom.createPortal(
        <>
            <div id='overlay'>
                <div id='modal'>
                    <div className='main'>
                        {children}
                    </div>
                    <div className='button-group'>
                        {buttons}
                        <button onClick={onClose}>Close</button>
                    </div>
                </div>

            </div>
        </>,
        document.getElementById('portal')
    )
}

export default Modal;
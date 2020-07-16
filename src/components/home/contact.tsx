import React, { useState } from "react";

import * as Form from "../../helpers/form"

const Contact = () => {
    let [status, set_status] = useState("");
    return (
        <div>
            <pre className="title">Contact Me</pre>
            <form 
                id="contact-form"
                onSubmit={e => Form.submit(e, set_status)}
            >
                <div id="entry-container">
                    <label className="entry">
                        <div className="entry-text">Your Email:</div>
                        <input type="text" name="_replyto" id="email" className="entry-input" placeholder="Email" required/>
                    </label>
                    <label className="entry">
                        <div className="entry-text">Your Name:</div>
                        <input type="text" name="name" className="entry-input" placeholder="Name" required/>
                    </label>
                </div>
                <label id="contact-message">
                    <div className="entry-text">Your message:</div>
                    <textarea name="message" style={{resize: "none"}} id="message-textarea" placeholder="Message" required></textarea>
                </label>
                <div id="contact-button-label">
                    <button type="submit" id="send-button">Send</button>
                    <div className="entry-text" style={{
                        marginLeft: "20px",
                        marginTop: "7px"
                    }}>{status}</div>
                </div>
            </form>
        </div>
    );
};

export default Contact;


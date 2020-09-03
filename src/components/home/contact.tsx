import React, { useState } from "react";

const Contact = () => {
    return (
        <div className="wrapper-center">
            <div className="wrapper-inner">
                <pre className="title">Contact Me</pre>
                <form 
                    id="contact-form"
                    method="POST"
                    data-netlify="true"
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
                    </div>
                </form>
            </div>
        </div>
    );
};

export default Contact;


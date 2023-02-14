-- Add migration script here
ALTER TABLE user_information 
ADD CONSTRAINT fk_otp_id
FOREIGN KEY (otp_id) 
REFERENCES one_time_passwords (id);
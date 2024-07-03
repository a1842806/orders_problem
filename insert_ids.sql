USE VSEC_DB;

DELIMITER $$

CREATE PROCEDURE insert_ids()
BEGIN
    DECLARE i INT DEFAULT 0;
    WHILE i < 7834 DO
        INSERT INTO IDs (user_id)
        VALUES (UUID());
        SET i = i + 1;
    END WHILE;
END$$

DELIMITER ;
CALL insert_ids();
DROP PROCEDURE IF EXISTS insert_ids;



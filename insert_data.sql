USE VSEC_DB;

DROP PROCEDURE IF EXISTS insert_data;

DELIMITER $$

CREATE PROCEDURE insert_data()
BEGIN
    DECLARE i INT DEFAULT 0;
    WHILE i < 100000 DO
        SET @uid = (SELECT user_id FROM IDs ORDER BY RAND() LIMIT 1);

        INSERT INTO Orders (timestamp, user_id, status)
        VALUES (NOW(), @uid, 
            CASE FLOOR(RAND() * 10)
                WHEN 0 THEN 'CLOSED'
                WHEN 1 THEN 'CANCELED'
                WHEN 2 THEN 'COMPLETE'
                WHEN 3 THEN 'PENDING_PAYMENT'
                WHEN 4 THEN 'SUSPECTED_FRAUD'
                WHEN 5 THEN 'PENDING'
                WHEN 6 THEN 'ON_HOLD'
                WHEN 7 THEN 'PROCESSING'
                WHEN 8 THEN 'PAYMENT_REVIEW'
                ELSE 'PENDING'
            END
        );
        SET i = i + 1;
    END WHILE;
END$$

DELIMITER ;
CALL insert_data();




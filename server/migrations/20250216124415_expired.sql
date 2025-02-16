-- Add migration script here
CREATE OR REPLACE FUNCTION remove_expired_challenges()
    RETURNS TRIGGER 
    LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM challenges WHERE status = 'pending' AND created_at < NOW() - INTERVAL '1 day';
    DELETE FROM challenges WHERE created_at < NOW() - INTERVAL '1 day';
    RETURN NEW;
END;
$$;

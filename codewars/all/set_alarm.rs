fn set_alarm(employed: bool, vacation: bool) -> bool
{
    employed && !vacation
    }

fn set_alarm1(employed: bool, vacation: bool) -> bool
{
    match(employed, vacation)
    {
        (true, false) => true,
        _             => false,
        }
    }
#[cfg(test)]
mod test
{
    use super::set_alarm;
    use super::set_alarm1;
    #[test]
    fn test_set_alaram()
    {
        assert_eq!(set_alarm(true, true), false);
        assert_eq!(set_alarm(false, true), false);
        assert_eq!(set_alarm(false, false), false);
        assert_eq!(set_alarm(true, false), true);
        assert_eq!(set_alarm1(true, true), false);
        assert_eq!(set_alarm1(false, true), false);
        assert_eq!(set_alarm1(false, false), false);
        assert_eq!(set_alarm1(true, false), true);
        }
    }
fn main() {
}

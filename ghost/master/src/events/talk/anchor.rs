/// 本文中の語句に張ったアンカーをクリックしたときに返す補足トーク。
/// 本文側は `\_a[AnchorTalk,<id>]語句\_a` の形で書き、ここに同じidで定義する。
/// 定義を足すときはこの関数を match に戻す。
pub(crate) fn anchor_talks(id: &str) -> Option<String> {
  debug!("Unknown anchor id: {}", id);
  None
}

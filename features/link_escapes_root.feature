Feature: recognize links that escape the root directory

  Background:
    Given file "one/alpha.md" with content:
      """
      # Alpha

      [Beta](../two/beta.md)
      """
    And file "two/beta.md" with content:
      """
      # Beta

      [Alpha](../one/alpha.md)
      """
    Then inspect the folder

  @this
  Scenario: check
    When checking in the "alpha" directory
    Then it finds no issues
    And all files are unchanged

Feature: recognize sections with inconsistent heading level

  Background:
    Given file "1.md" with content:
      """
      # One

      ### alpha
      [Two](2.md)
      [Three](3.md)
      """
    And file "2.md" with content:
      """
      # Two

      ##### alpha
      [One](1.md)
      """
    And file "3.md" with content:
      """
      # Three

      ### alpha
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      2.md:3  heading level (<h5>) is inconsistent with the usual level for "alpha" (<h3>)
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      2.md:3  normalized section "alpha" from <h5> to <h3>
      """
    And file "2.md" should contain:
      """
      # Two

      ### alpha
      [One](1.md)
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      2.md:3  normalized section "alpha" from <h5> to <h3>
      """
    And file "2.md" should contain:
      """
      # Two

      ### alpha
      [One](1.md)
      """
    And the exit code is 0

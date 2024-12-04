package day04;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_4 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	public static void main(String[] args) {
		Path path = Path.of("src/day04/Task04.txt");

		int result = 0;
		int result2 = 0;
		int ex = 0;

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());

			char[][] matrix = new char[150][150];
			int k = 0;

			for (String line : inputList) {
				matrix[k] = line.toCharArray();
				k++;
			}

			for (int i = 0; i < 140; i++) {
				for (int j = 0; j < 140; j++) {

					if ('X' == matrix[i][j]) {
						try {
							if (i >= 3) {
								String upStr = "" + 'X' + matrix[i - 1][j] + matrix[i - 2][j] + matrix[i - 3][j];
								if (check(upStr))
									result++;
							}
							if (i <= 136) {
								String downStr = "" + 'X' + matrix[i + 1][j] + matrix[i + 2][j] + matrix[i + 3][j];
								if (check(downStr))
									result++;
							}
							if (j >= 3) {
								String rightStr = "" + 'X' + matrix[i][j - 1] + matrix[i][j - 2] + matrix[i][j - 3];
								if (check(rightStr))
									result++;
							}
							if (j <= 136) {
								String leftStr = "" + 'X' + matrix[i][j + 1] + matrix[i][j + 2] + matrix[i][j + 3];
								if (check(leftStr))
									result++;
							}
							if ((i >= 3) && (j >= 3)) {
								String upLeft = "" + 'X' + matrix[i - 1][j - 1] + matrix[i - 2][j - 2]
										+ matrix[i - 3][j - 3];
								if (check(upLeft))
									result++;
							}
							if ((i <= 136) && (j >= 3)) {
								String downLeft = "" + 'X' + matrix[i + 1][j - 1] + matrix[i + 2][j - 2]
										+ matrix[i + 3][j - 3];
								if (check(downLeft)) {
									result++;
								}
							}
							if ((i >= 3) && (j <= 136)) {
								String upRight = "" + 'X' + matrix[i - 1][j + 1] + matrix[i - 2][j + 2]
										+ matrix[i - 3][j + 3];
								if (check(upRight)) {
									result++;
								}

							}
							if ((i <= 136) && (j <= 136)) {
								String downRight = "" + 'X' + matrix[i + 1][j + 1] + matrix[i + 2][j + 2]
										+ matrix[i + 3][j + 3];
								if (check(downRight)) {
									result++;
								}
							}

						} catch (Exception e) {
							ex++;
						}

					}
				}
			}

			// ----------------------------------//

			for (int i = 0; i < 140; i++) {
				for (int j = 0; j < 140; j++) {

					if ('A' == matrix[i][j]) {
						try {
							if ((i >= 1) && (j >= 1) && (i <= 138) && (j <= 138)) {
								String upLeft = "" + matrix[i - 1][j - 1] + 'A' + matrix[i + 1][j + 1];
								String downLeft = "" + matrix[i + 1][j - 1] + 'A' + matrix[i - 1][j + 1];
								if (check2(upLeft) && check2(downLeft))
									result2++;
							}
							if ((i >= 1) && (j >= 1) && (i <= 138) && (j <= 138)) {
								String upLeft = "" + matrix[i - 1][j - 1] + 'A' + matrix[i + 1][j + 1];
								String upRight = "" + matrix[i - 1][j + 1] + 'A' + matrix[i + 1][j - 1];
								if (check2(upLeft) && check2(upRight)) {
									result2++;
								}
							}
							if ((i >= 1) && (j >= 1) && (i <= 138) && (j <= 138)) {
								String upRight = "" + matrix[i - 1][j + 1] + 'A' + matrix[i + 1][j - 1];
								String downRight = "" + matrix[i + 1][j + 1] + 'A' + matrix[i - 1][j - 1];
								if (check2(upRight) && check2(downRight)) {
									result2++;
								}

							}
							if ((i >= 1) && (j >= 1) && (i <= 138) && (j <= 138)) {
								String downRight = "" + matrix[i + 1][j + 1] + 'A' + matrix[i - 1][j - 1];
								String downLeft = "" + matrix[i + 1][j - 1] + 'A' + matrix[i - 1][j + 1];
								if (check2(downRight) && check2(downLeft)) {
									result2++;
								}
							}

						} catch (Exception e) {
							ex++;
						}

					}
				}
			}

			System.out.println(result);
			System.out.println(result2);
			// System.out.println(ex);

		} catch (

		IOException e) {
			ex++;
			System.out.println("Error");
		}

	}

	private static boolean check(String str) {
		if ("XMAS".equals(str)) {
			return true;
		} else {
			return false;
		}
	}

	private static boolean check2(String str) {
		if ("MAS".equals(str)) {
			return true;
		} else {
			return false;
		}
	}

}

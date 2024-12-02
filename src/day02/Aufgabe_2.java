package day02;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_2 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	public static void main(String[] args) {
		Path path = Path.of("src/day02/Task02.txt");

		int counter = 0;
		int counter2 = 0;

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());
			// inputList.add("");
//			inputList2.add("1 3 4 5 8 10 7\r\n" + "48 51 54 55 55\r\n" + "7 9 12 15 16 17 20 24\r\n"
//					+ "49 52 55 57 58 60 65\r\n" + "73 75 77 80 81 78 81 82\r\n" + "83 84 87 85 86 84\r\n"
//					+ "76 77 80 79 80 80\r\n" + "25 27 28 29 28 30 34\r\n" + "21 23 26 28 26 27 34\r\n"
//					+ "71 72 72 73 74 75 77 80");
			// System.out.println(inputList);

			for (String line : inputList) {
				int[] num = new int[10];
				innerLoop: for (int i = 0; i < 10; i++) {
					if (extract(line).size() == i) {
						break innerLoop;
					}
					num[i] = extract(line).get(i);
				}

				if ((ascSave(num)) || (descSave(num))) {
					counter++;
				} else {
					continue;
				}

			}

			System.out.println(counter);

			// ---------------------------//

			for (String line : inputList2) {
				int[] num = new int[10];
				innerLoop: for (int i = 0; i < 10; i++) {
					if (extract(line).size() == i) {
						break innerLoop;
					}
					num[i] = extract(line).get(i);
				}

				if ((ascSave2(num)) || (descSave2(num))) {
					counter2++;
				} else {
					continue;
				}

			}

			System.out.println(counter2);

		} catch (IOException e) {
			System.out.println("Error");
		}
	}

	public static ArrayList<Integer> extract(String i) {
		Pattern pattern = Pattern.compile("\\d+");
		Matcher matcher = pattern.matcher(i);

		ArrayList<Integer> n = new ArrayList<>();
		while (matcher.find()) {
			n.add(Integer.parseInt(matcher.group()));
		}
		return (ArrayList<Integer>) n;
	}

	public static boolean ascSave(int[] arr) {

		for (int i = 0; i < 10; i++) {
			if (arr[i + 1] == 0) {
				break;
			}
			if ((arr[i] > arr[i + 1]) || (Math.abs(arr[i] - arr[i + 1]) > 3) || (Math.abs(arr[i] - arr[i + 1]) < 1)) {
				return false;
			}

		}
		return true;
	}

	public static boolean descSave(int[] arr) {

		for (int i = 0; i < 10; i++) {
			if (arr[i + 1] == 0) {
				break;
			}
			if ((arr[i] < arr[i + 1]) || (Math.abs(arr[i] - arr[i + 1]) > 3) || (Math.abs(arr[i] - arr[i + 1]) < 1)) {
				return false;
			}

		}
		return true;
	}

	public static boolean ascSave2(int[] arr) {
		int errorCount = 0;
		int[] nutz = arr;

		for (int i = 0; i < 10; i++) {
			if (nutz[i + 1] == 0) {
				break;
			}
			if ((nutz[i] > nutz[i + 1]) || (Math.abs(nutz[i] - nutz[i + 1]) > 3)
					|| (Math.abs(nutz[i] - nutz[i + 1]) < 1)) {
				errorCount++;
				if (errorCount > 1) {
					return false;
				}
				nutz = removeOneElement(nutz, i + 1);
				// i = 0;
				i--;
			}
		}

		return true;

	}

	public static boolean descSave2(int[] arr) {
		int errorCount = 0;
		int[] nutz = arr;

		for (int i = 0; i < 10; i++) {
			if (nutz[i + 1] == 0) {
				break;
			}
			if ((nutz[i] < nutz[i + 1]) || (Math.abs(nutz[i] - nutz[i + 1]) > 3)
					|| (Math.abs(nutz[i] - nutz[i + 1]) < 1)) {
				errorCount++;
				if (errorCount > 1) {
					return false;
				}
				nutz = removeOneElement(nutz, i + 1);
				// i = 0;
				i--;
			}

		}
		return true;
	}

	public static int[] removeOneElement(int[] line, int index) {
		int[] newLine = new int[line.length - 1];

		for (int i = 0, k = 0; i < line.length; i++) {
			if (i == index) {
				continue;
			}

			newLine[k++] = line[i];
		}

		return newLine;
	}

}

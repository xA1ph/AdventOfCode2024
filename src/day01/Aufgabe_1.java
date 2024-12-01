package day01;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_1 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	public static void main(String[] args) {
		Path path = Path.of("src/day01/Task01.txt");

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());

			int[] leftArray = new int[1000];
			int[] rightArray = new int[1000];

			for (int i = 0; i < 1000; i++) {
				leftArray[i] = Integer.parseInt(inputList.get(i).substring(0, 5));
				rightArray[i] = Integer.parseInt(inputList.get(i).substring(8, 13));
			}

			BubbleSort(leftArray);
			BubbleSort(rightArray);

			int result = 0;

			for (int i = 0; i < 1000; i++) {
				int num = leftArray[i] - rightArray[i];
				num = Math.abs(num);

				result += num;
			}

			System.out.println(result);

			// -------------------------------------//

			ArrayList<Integer> leftList = new ArrayList<>();
			ArrayList<Integer> rightList = new ArrayList<>();

			for (String line : inputList2) {
				leftList.add(Integer.parseInt(line.substring(0, 5)));
				rightList.add(Integer.parseInt(line.substring(8, 13)));
			}

			int result2 = 0;

			for (Integer left : leftList) {
				int count = 0;

				for (Integer right : rightList) {
					if (left.equals(right)) {
						count++;
					}
				}

				int zahl = left * count;

				result2 += zahl;
			}

			System.out.println(result2);

		} catch (IOException e) {
			System.out.println("Error");
		}
	}

	public static void BubbleSort(int[] arr) {

		for (int n = arr.length; n >= 2; n--) {
			for (int i = 0; i <= n - 2; i++) {

				if (arr[i] > arr[i + 1]) {

					// vertauschen
					int tmp = arr[i];
					arr[i] = arr[i + 1];
					arr[i + 1] = tmp;
				}
			}
		}

	}

}

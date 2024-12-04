package day03;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_3 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	public static void main(String[] args) {
		Path path = Path.of("src/day03/Task03.txt");

		int result = 0;
		int result2 = 0;

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());

			Pattern pattern1 = Pattern.compile("mul\\(\\d{1,3},\\d{1,3}\\)");
			Pattern pattern12 = Pattern.compile("mul\\(\\d{1,3},\\d{1,3}\\)|do\\(\\)|don't\\(\\)");
			Pattern pattern2 = Pattern.compile("\\d{1,3}");

			for (String line : inputList) {
				Matcher matcher1 = pattern1.matcher(line);

				while (matcher1.find()) {
					String mul = matcher1.group();
					Matcher matcher2 = pattern2.matcher(mul);

					Integer x = 0;
					Integer y = 0;

					if (matcher2.find()) {
						x = Integer.valueOf(matcher2.group());
					}
					if (matcher2.find()) {
						y = Integer.valueOf(matcher2.group());
					}

					result += x * y;
				}
			}

			System.out.println(result);

			// --------------------------------//

			boolean isActive = true;

			for (String line : inputList2) {
				Matcher matcher1 = pattern12.matcher(line);

				while (matcher1.find()) {
					String mul = matcher1.group();

					if (mul.equals("don't()")) {
						isActive = false;
					}
					if (mul.equals("do()")) {
						isActive = true;
					}
					if (!(mul.equals("don't()")) && !(mul.equals("do()")) && (isActive)) {
						Matcher matcher2 = pattern2.matcher(mul);

						Integer x = 0;
						Integer y = 0;

						if (matcher2.find()) {
							x = Integer.valueOf(matcher2.group());
						}
						if (matcher2.find()) {
							y = Integer.valueOf(matcher2.group());
						}

						result2 += x * y;
					}
				}
			}

			System.out.println(result2);

		} catch (IOException e) {
			System.out.println("Error");
		}
	}
}
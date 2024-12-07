package day06;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Vector;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Aufgabe_6 {

	private static List<String> inputList = new ArrayList<>();
	private static List<String> inputList2 = new ArrayList<>();

	private static char[][] matrix = new char[130][130];
	private static Vector<Integer> curr = new Vector<>();

	public static void main(String[] args) {
		Path path = Path.of("src/day06/Task06.txt");

		int result = 0;
		int result2 = 0;

		try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
			inputList = input.collect(Collectors.toList());
			inputList2 = input2.collect(Collectors.toList());

			int k = 0;
			for (String line : inputList) {
				matrix[k] = line.toCharArray();
				k++;
			}

			curr = getCurrPos();
			int i = curr.getFirst();
			int j = curr.getLast();
			while (true) {
				curr = getCurrPos();
				while (!wallNorth(curr)) {
					walk("N");
					i--;
				}
				curr = getCurrPos();
				while (!wallEast(curr)) {
					walk("E");
					j++;
				}
				curr = getCurrPos();
				while (!wallSouth(curr)) {
					walk("S");
					i++;
				}
				curr = getCurrPos();
				while (!wallWest(curr)) {
					walk("W");
					j--;
				}

			}

			// ------------------------------------- //

		} catch (IOException e) {
			System.out.println("Error");
		} catch (Exception e) {
			System.out.println(findAllPos() + 1);
		}

	}

	private static Vector<Integer> getCurrPos() {
		Vector<Integer> curr = new Vector<>();
		outerloop: for (int i = 0; i < 130; i++) {
			for (int j = 0; j < 130; j++) {
				if (matrix[i][j] == '^') {
					curr.addFirst(i);
					curr.addLast(j);
					break outerloop;
				}
			}
		}
		return curr;
	}

	private static void walk(String dir) {
		if ("N".equals(dir)) {
			curr.set(0, curr.getFirst() - 1);
			curr.set(1, curr.getLast());
			matrix[curr.getFirst()][curr.getLast()] = '^';
			matrix[curr.getFirst() + 1][curr.getLast()] = 'X';
		}
		if ("E".equals(dir)) {
			curr.set(0, curr.getFirst());
			curr.set(1, curr.getLast() + 1);
			matrix[curr.getFirst()][curr.getLast()] = '^';
			matrix[curr.getFirst()][curr.getLast() - 1] = 'X';
		}
		if ("S".equals(dir)) {
			curr.set(0, curr.getFirst() + 1);
			curr.set(1, curr.getLast());
			matrix[curr.getFirst()][curr.getLast()] = '^';
			matrix[curr.getFirst() - 1][curr.getLast()] = 'X';
		}
		if ("W".equals(dir)) {
			curr.set(0, curr.getFirst());
			curr.set(1, curr.getLast() - 1);
			matrix[curr.getFirst()][curr.getLast()] = '^';
			matrix[curr.getFirst()][curr.getLast() + 1] = 'X';
		}

	}

	private static boolean wallNorth(Vector<Integer> curr) {
		if (matrix[curr.getFirst() - 1][curr.getLast()] == '#') {
			return true;
		}
		return false;
	}

	private static boolean wallEast(Vector<Integer> curr) {
		if (matrix[curr.getFirst()][curr.getLast() + 1] == '#') {
			return true;
		}
		return false;
	}

	private static boolean wallSouth(Vector<Integer> curr) {
		if (matrix[curr.getFirst() + 1][curr.getLast()] == '#') {
			return true;
		}
		return false;
	}

	private static boolean wallWest(Vector<Integer> curr) {
		if (matrix[curr.getFirst()][curr.getLast() - 1] == '#') {
			return true;
		}
		return false;
	}

	private static int findAllPos() {
		int result = 0;
		for (int i = 0; i < 130; i++) {
			for (int j = 0; j < 130; j++) {
				if (matrix[i][j] == 'X') {
					result++;
				}
			}
		}
		return result;
	}
}

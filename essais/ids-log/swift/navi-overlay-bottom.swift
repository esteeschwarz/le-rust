//
//  naviwtoverlaybottom.swift
//  MyProject
//
//  Designed in DetailsPro
//  Copyright © (My Organization). All rights reserved.
//

import SwiftUI

struct naviwtoverlaybottom: View {
    var body: some View {
		ScrollView {
			VStack {
				Text("Title")
					.font(.system(.largeTitle, weight: .bold))
					.frame(maxWidth: .infinity, alignment: .leading)
					.clipped()
					.padding(.leading)
					.padding(.bottom, 8)
			}
			.frame(maxWidth: .infinity)
			.clipped()
			.padding(.top, 98)
			.padding(.bottom, 150)
		}
		.overlay(alignment: .top) {
			Group {
				
			}
		}
		.overlay(alignment: .bottom) {
			// Tab Bar
			VStack(spacing: 0) {
				Divider()
				HStack(spacing: 10) {
					ForEach(0..<5) { _ in // Replace with your data model here
						VStack(spacing: 4) {
							Image(systemName: "play.circle.fill")
								.imageScale(.large)
								.frame(height: 26)
								.clipped()
							Text("Listen Now")
								.font(.caption2)
						}
						.frame(maxWidth: .infinity)
						.clipped()
						.frame(height: 45)
						.clipped()
						.foregroundStyle(.secondary)
					}
				}
				.padding(.horizontal, 15)
				.padding(.top, 5)
			}
			.frame(height: 84, alignment: .top)
			.clipped()
			.background {
				Rectangle()
					.fill(.clear)
					.background(Material.bar)
			}
		}
    }
}

#Preview {
    naviwtoverlaybottom()
}